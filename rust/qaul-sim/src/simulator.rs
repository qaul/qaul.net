//! Discrete-time simulator that drives real libqaul RouterState instances.
//!
//! Each "tick" simulates one routing period:
//! 1. For each link, simulate ping → update neighbour RTT
//! 2. For each node, create routing info and deliver to neighbours
//! 3. For each node, build the global routing table from connection tables
//!
//! All operations call real libqaul production code — no mocking.

use libp2p::identity::Keypair;
use libp2p::PeerId;
use rand::Rng;
use std::sync::Arc;

use libqaul::connections::ConnectionModule;
use libqaul::router::RouterState;
use libqaul::storage::configuration::RoutingOptions;

use crate::metrics::Metrics;
use crate::network;
use crate::topology::Topology;

/// A simulated node wrapping a real RouterState.
pub struct SimNode {
    /// The node's libp2p PeerId.
    pub peer_id: PeerId,
    /// The node's router state (owns all routing tables).
    pub router: Arc<RouterState>,
}

/// The main simulator.
pub struct Simulator {
    /// Simulated nodes.
    pub nodes: Vec<SimNode>,
    /// Network topology.
    pub topology: Topology,
    /// Router configuration shared by all nodes.
    pub config: RoutingOptions,
    /// Current tick number.
    pub tick: u64,
}

impl Simulator {
    /// Create a new simulator from a topology.
    /// Generates fresh PeerIds for each node and creates RouterState instances.
    pub fn new(topology: Topology, config: RoutingOptions) -> Self {
        let mut nodes = Vec::with_capacity(topology.node_count);

        for _ in 0..topology.node_count {
            let keypair = Keypair::generate_ed25519();
            let peer_id = keypair.public().to_peer_id();
            let router = Arc::new(RouterState::new(config.clone()));

            // Register this node as a "local user" in its own routing table
            router
                .connections
                .add_local_user(peer_id, peer_id);

            nodes.push(SimNode {
                peer_id,
                router,
            });
        }

        Self {
            nodes,
            topology,
            config,
            tick: 0,
        }
    }

    /// Run one tick of the simulation:
    /// 1. Ping: update neighbour RTT tables for all active links
    /// 2. Exchange: create routing info on each node and deliver to neighbours
    /// 3. Build: recalculate the global routing table on each node
    pub fn tick(&mut self, rng: &mut impl Rng) {
        self.tick += 1;

        // Phase 1: Ping — update neighbour tables
        self.phase_ping(rng);

        // Phase 2: Exchange routing info
        self.phase_exchange();

        // Phase 3: Build routing tables
        self.phase_build();
    }

    /// Phase 1: For each active link, update neighbour RTTs on both endpoints.
    fn phase_ping(&self, rng: &mut impl Rng) {
        for (&(a, b), link) in &self.topology.links {
            if !link.active {
                continue;
            }

            if let Some(rtt) = network::sample_rtt(link, rng) {
                let peer_a = self.nodes[a].peer_id;
                let peer_b = self.nodes[b].peer_id;

                // Node A sees Node B as a LAN neighbour
                self.nodes[a]
                    .router
                    .neighbours
                    .update_node(ConnectionModule::Lan, peer_b, rtt);

                // Add B to A's scheduler
                self.nodes[a].router.scheduler.add_neighbour(peer_b);

                // Node B sees Node A as a LAN neighbour
                self.nodes[b]
                    .router
                    .neighbours
                    .update_node(ConnectionModule::Lan, peer_a, rtt);

                // Add A to B's scheduler
                self.nodes[b].router.scheduler.add_neighbour(peer_a);
            }
        }
    }

    /// Phase 2: Each node creates routing info and delivers it to all neighbours.
    fn phase_exchange(&self) {
        // Collect routing info from each node for each of its neighbours.
        // We collect first to avoid borrow conflicts.
        let mut deliveries: Vec<(usize, PeerId, Vec<libqaul::router::router_net_proto::RoutingInfoEntry>)> = Vec::new();

        for node_idx in 0..self.nodes.len() {
            let node = &self.nodes[node_idx];
            let neighbours = self.topology.neighbours(node_idx);

            for (neighbour_idx, _link) in neighbours {
                let neighbour_peer = self.nodes[neighbour_idx].peer_id;

                // Create routing info to send to this neighbour
                let info = node
                    .router
                    .routing_table
                    .create_routing_info(neighbour_peer, 0);

                if !info.entry.is_empty() {
                    deliveries.push((neighbour_idx, node.peer_id, info.entry));
                }
            }
        }

        // Deliver all routing info
        for (dest_idx, sender_peer, entries) in deliveries {
            let dest = &self.nodes[dest_idx];
            dest.router.connections.process_received_routing_info(
                sender_peer,
                &entries,
                &dest.router.neighbours,
                &self.config,
            );
        }
    }

    /// Phase 3: Each node builds its global routing table from connection tables.
    fn phase_build(&self) {
        for node in &self.nodes {
            let new_table = node.router.connections.create_routing_table(&self.config);
            node.router.routing_table.set(new_table);
        }
    }

    /// Run `n` ticks.
    pub fn run(&mut self, n: u64, rng: &mut impl Rng) {
        for _ in 0..n {
            self.tick(rng);
        }
    }

    /// Collect metrics about the current state of the simulation.
    pub fn metrics(&self) -> Metrics {
        Metrics::collect(self)
    }

    /// Check if all nodes can reach all other nodes.
    pub fn is_fully_converged(&self) -> bool {
        let n = self.nodes.len();
        for i in 0..n {
            let table = self.nodes[i].router.routing_table.inner.read().unwrap();
            // Node should know about all other nodes (n-1 entries, not counting self)
            let reachable = table
                .table
                .values()
                .filter(|e| !e.connections.is_empty())
                .count();
            // We need at least n-1 reachable users (self is already in local)
            if reachable < n {
                return false;
            }
        }
        true
    }

    /// Get the number of ticks until convergence, running up to `max_ticks`.
    /// Returns `None` if convergence is not reached.
    pub fn ticks_to_convergence(&mut self, max_ticks: u64, rng: &mut impl Rng) -> Option<u64> {
        for t in 0..max_ticks {
            self.tick(rng);
            if self.is_fully_converged() {
                return Some(t + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::Topology;

    fn default_config() -> RoutingOptions {
        RoutingOptions {
            sending_table_period: 10,
            ping_neighbour_period: 5,
            hop_count_penalty: 10,
            maintain_period_limit: 300,
        }
    }

    #[test]
    fn two_node_direct_link() {
        let topo = Topology::line(2, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        // After a few ticks, both nodes should see each other
        sim.run(3, &mut rng);

        // Node 0 should have node 1 in its routing table and vice versa
        let table0 = sim.nodes[0].router.routing_table.inner.read().unwrap();
        let table1 = sim.nodes[1].router.routing_table.inner.read().unwrap();

        // Both should have 2 entries (self + other)
        let reachable0 = table0.table.values().filter(|e| !e.connections.is_empty()).count();
        let reachable1 = table1.table.values().filter(|e| !e.connections.is_empty()).count();

        assert_eq!(reachable0, 2, "Node 0 should see 2 users (self + node 1)");
        assert_eq!(reachable1, 2, "Node 1 should see 2 users (self + node 0)");
    }

    #[test]
    fn three_node_line_convergence() {
        // 0 -- 1 -- 2
        let topo = Topology::line(3, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        // Need multiple ticks for multi-hop route to propagate
        let ticks = sim.ticks_to_convergence(20, &mut rng);
        assert!(
            ticks.is_some(),
            "3-node line should converge within 20 ticks"
        );
        assert!(ticks.unwrap() <= 10, "Should converge quickly");
    }

    #[test]
    fn ring_convergence() {
        let topo = Topology::ring(5, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "5-node ring should converge within 30 ticks");
    }

    #[test]
    fn grid_convergence() {
        let topo = Topology::grid(3, 3, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "3x3 grid should converge within 30 ticks");
    }

    #[test]
    fn link_failure_and_recovery() {
        let topo = Topology::line(3, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        // Converge first
        sim.run(10, &mut rng);
        assert!(sim.is_fully_converged());

        // Break the link between 0 and 1
        sim.topology.set_link_active(0, 1, false);

        // The existing routes will still be in the table but will eventually expire.
        // For now, just verify the link is broken.
        let neighbours = sim.topology.neighbours(0);
        assert_eq!(neighbours.len(), 0, "Node 0 should have no active neighbours");

        // Restore the link
        sim.topology.set_link_active(0, 1, true);
        sim.run(5, &mut rng);

        // Node 0 should eventually see nodes again after re-convergence
        let table0 = sim.nodes[0].router.routing_table.inner.read().unwrap();
        let reachable = table0.table.values().filter(|e| !e.connections.is_empty()).count();
        assert!(reachable >= 2, "Node 0 should see at least itself and node 1 after recovery");
    }

    #[test]
    fn full_mesh_immediate_convergence() {
        let topo = Topology::full_mesh(4, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        // Full mesh should converge very quickly since everyone is a direct neighbour
        let ticks = sim.ticks_to_convergence(5, &mut rng);
        assert!(ticks.is_some(), "4-node full mesh should converge within 5 ticks");
        assert!(ticks.unwrap() <= 3, "Full mesh should converge in 2-3 ticks");
    }
}
