//! Metrics collection for simulation analysis.
//!
//! Collects convergence, route quality, and stability information
//! from a running simulation.

use crate::simulator::Simulator;

/// Snapshot of simulation metrics at a point in time.
#[derive(Debug, Clone)]
pub struct Metrics {
    /// Current tick number.
    pub tick: u64,
    /// Number of nodes in the simulation.
    pub node_count: usize,
    /// Whether all nodes can reach all other nodes.
    pub fully_converged: bool,
    /// Per-node: how many other nodes are reachable.
    pub reachability: Vec<usize>,
    /// Average reachability across all nodes.
    pub avg_reachability: f64,
    /// Total number of routing table entries across all nodes.
    pub total_routes: usize,
    /// Average hop count across all reachable routes (0 = no routes).
    pub avg_hop_count: f64,
    /// Maximum hop count across all reachable routes.
    pub max_hop_count: u32,
    /// Number of routes per connection module (module name -> count).
    pub routes_by_module: Vec<(String, usize)>,
}

impl Metrics {
    /// Collect metrics from the current simulator state.
    pub fn collect(sim: &Simulator) -> Self {
        use std::collections::HashMap;

        let n = sim.nodes.len();
        let mut reachability = Vec::with_capacity(n);
        let mut total_routes = 0;
        let mut hop_sum: u64 = 0;
        let mut hop_count_entries: u64 = 0;
        let mut max_hop_count: u32 = 0;
        let mut module_counts: HashMap<String, usize> = HashMap::new();

        for node in &sim.nodes {
            let table = node.router.routing_table.inner.read().unwrap();
            let reachable = table
                .table
                .values()
                .filter(|e| !e.connections.is_empty())
                .count();
            reachability.push(reachable);
            total_routes += table.table.len();

            // Analyze hop counts and connection modules
            for entry in table.table.values() {
                for conn in &entry.connections {
                    let hops = conn.hc as u32;
                    hop_sum += hops as u64;
                    hop_count_entries += 1;
                    if hops > max_hop_count {
                        max_hop_count = hops;
                    }

                    let module_name = conn.module.as_str_name().to_string();
                    *module_counts.entry(module_name).or_insert(0) += 1;
                }
            }
        }

        let avg_reachability = if n > 0 {
            reachability.iter().sum::<usize>() as f64 / n as f64
        } else {
            0.0
        };

        let avg_hop_count = if hop_count_entries > 0 {
            hop_sum as f64 / hop_count_entries as f64
        } else {
            0.0
        };

        let fully_converged = reachability.iter().all(|&r| r >= n);

        let mut routes_by_module: Vec<(String, usize)> = module_counts.into_iter().collect();
        routes_by_module.sort_by(|a, b| a.0.cmp(&b.0));

        Self {
            tick: sim.tick,
            node_count: n,
            fully_converged,
            reachability,
            avg_reachability,
            total_routes,
            avg_hop_count,
            max_hop_count,
            routes_by_module,
        }
    }

    /// Print a human-readable summary.
    pub fn summary(&self) -> String {
        let mut s = format!(
            "tick={}, nodes={}, converged={}, avg_reach={:.1}, total_routes={}, avg_hops={:.2}, max_hops={}",
            self.tick,
            self.node_count,
            self.fully_converged,
            self.avg_reachability,
            self.total_routes,
            self.avg_hop_count,
            self.max_hop_count,
        );
        if !self.routes_by_module.is_empty() {
            s.push_str(", modules={");
            for (i, (name, count)) in self.routes_by_module.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&format!("{}:{}", name, count));
            }
            s.push('}');
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use crate::simulator::Simulator;
    use crate::topology::Topology;
    use libqaul::storage::configuration::RoutingOptions;

    fn default_config() -> RoutingOptions {
        RoutingOptions {
            sending_table_period: 10,
            ping_neighbour_period: 5,
            hop_count_penalty: 10,
            maintain_period_limit: 300,
        }
    }

    #[test]
    fn metrics_before_any_ticks() {
        let topo = Topology::line(3, 5000);
        let sim = Simulator::new(topo, default_config());
        let m = sim.metrics();

        assert_eq!(m.tick, 0);
        assert_eq!(m.node_count, 3);
        // Before any ticks, the routing table hasn't been built yet.
        // Local users are in connection tables but not in the routing table.
        assert!(!m.fully_converged);
    }

    #[test]
    fn metrics_after_convergence() {
        let topo = Topology::line(3, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();

        sim.run(15, &mut rng);
        let m = sim.metrics();

        assert!(m.fully_converged, "Should be converged after 15 ticks");
        assert_eq!(m.avg_reachability, 3.0);
    }
}
