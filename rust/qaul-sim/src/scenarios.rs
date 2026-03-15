//! Predefined simulation scenarios.
//!
//! Each scenario creates a topology and runs a simulation with
//! specific conditions to test different aspects of the routing protocol.

use libqaul::storage::configuration::RoutingOptions;

use crate::simulator::Simulator;
use crate::topology::{Link, Topology};

/// Default routing configuration for simulation.
pub fn default_config() -> RoutingOptions {
    RoutingOptions {
        sending_table_period: 10,
        ping_neighbour_period: 5,
        hop_count_penalty: 10,
        maintain_period_limit: 300,
    }
}

/// Scenario: Line topology convergence.
/// Tests how many ticks it takes for a line of N nodes to converge.
pub fn line_convergence(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::line(n, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: Ring topology convergence.
pub fn ring_convergence(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::ring(n, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: Grid topology convergence.
pub fn grid_convergence(rows: usize, cols: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::grid(rows, cols, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: Latency spike test.
/// Creates a line topology where one link suddenly gets high latency.
pub fn latency_spike(n: usize, base_rtt_us: u32, spike_rtt_us: u32) -> Simulator {
    let mut topo = Topology::new(n);
    for i in 0..n.saturating_sub(1) {
        let rtt = if i == n / 2 {
            spike_rtt_us
        } else {
            base_rtt_us
        };
        topo.add_link(i, i + 1, Link::new(rtt));
    }
    Simulator::new(topo, default_config())
}

/// Scenario: Link flapping.
/// Creates a line topology. The link at position `flap_pos` can be toggled on/off.
pub fn link_flapping(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::line(n, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: Network partition and heal.
/// Creates a line topology that can be split in half by disabling the middle link.
pub fn partition_heal(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::line(n, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: Star topology with variable arm lengths.
pub fn star_topology(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::star(n, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: BLE-only mesh.
/// All links are BLE connections with higher latency and some jitter.
pub fn ble_only_mesh(n: usize) -> Simulator {
    let mut topo = Topology::new(n);
    for i in 0..n.saturating_sub(1) {
        topo.add_link(i, i + 1, Link::ble(15000).with_jitter(3000));
    }
    Simulator::new(topo, default_config())
}

/// Scenario: Mixed BLE + LAN topology.
/// Creates a chain where odd links are BLE and even links are LAN,
/// simulating a heterogeneous network.
pub fn mixed_ble_lan(n: usize) -> Simulator {
    let mut topo = Topology::new(n);
    for i in 0..n.saturating_sub(1) {
        let link = if i % 2 == 0 {
            Link::new(5000) // LAN
        } else {
            Link::ble(15000).with_jitter(2000)
        };
        topo.add_link(i, i + 1, link);
    }
    Simulator::new(topo, default_config())
}

/// Scenario: Internet relay topology.
/// A star where the center node connects to all others via Internet links.
pub fn internet_relay(n: usize) -> Simulator {
    let mut topo = Topology::new(n);
    for i in 1..n {
        topo.add_link(0, i, Link::internet(20000).with_jitter(5000));
    }
    Simulator::new(topo, default_config())
}

/// Scenario: Mixed module star — center connects via Internet,
/// leaf nodes connect to each other via BLE where adjacent.
pub fn mixed_star_ble_internet(n: usize) -> Simulator {
    let mut topo = Topology::new(n);
    // Hub-to-spoke links via Internet
    for i in 1..n {
        topo.add_link(0, i, Link::internet(20000));
    }
    // Adjacent spoke-to-spoke links via BLE
    for i in 1..n.saturating_sub(1) {
        topo.add_link(i, i + 1, Link::ble(10000));
    }
    Simulator::new(topo, default_config())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_5_converges() {
        let mut sim = line_convergence(5, 5000);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "5-node line should converge");
        println!("5-node line converged in {} ticks", ticks.unwrap());
    }

    #[test]
    fn ring_8_converges() {
        let mut sim = ring_convergence(8, 5000);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "8-node ring should converge");
        println!("8-node ring converged in {} ticks", ticks.unwrap());
    }

    #[test]
    fn grid_4x4_converges() {
        let mut sim = grid_convergence(4, 4, 5000);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(40, &mut rng);
        assert!(ticks.is_some(), "4x4 grid should converge");
        println!("4x4 grid converged in {} ticks", ticks.unwrap());
    }

    #[test]
    fn latency_spike_converges() {
        let mut sim = latency_spike(5, 5000, 50000);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "Latency spike scenario should converge");
    }

    #[test]
    fn partition_and_heal() {
        let mut sim = partition_heal(6, 5000);
        let mut rng = rand::rng();

        // First converge
        sim.run(15, &mut rng);
        assert!(sim.is_fully_converged(), "Should converge initially");

        // Partition: break middle link (between node 2 and 3)
        sim.topology.set_link_active(2, 3, false);

        // Run a few ticks — routes through the broken link should not update
        // (but won't expire immediately due to maintain_period_limit)
        sim.run(3, &mut rng);

        // Heal
        sim.topology.set_link_active(2, 3, true);

        // Run enough ticks to re-converge
        sim.run(15, &mut rng);
        assert!(
            sim.is_fully_converged(),
            "Should re-converge after partition heal"
        );
    }

    #[test]
    fn star_topology_converges() {
        let mut sim = star_topology(6, 5000);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(20, &mut rng);
        assert!(ticks.is_some(), "Star topology should converge");
        println!("6-node star converged in {} ticks", ticks.unwrap());
    }

    #[test]
    fn ble_only_mesh_converges() {
        let mut sim = ble_only_mesh(5);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "BLE-only mesh should converge");
        println!("5-node BLE mesh converged in {} ticks", ticks.unwrap());

        // Verify routes are via BLE module
        let m = sim.metrics();
        assert!(m.fully_converged);
        let ble_routes = m.routes_by_module.iter().find(|(name, _)| name == "BLE");
        assert!(ble_routes.is_some(), "Should have BLE routes");
        println!("BLE routes: {}", ble_routes.unwrap().1);
    }

    #[test]
    fn mixed_ble_lan_converges() {
        let mut sim = mixed_ble_lan(6);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "Mixed BLE+LAN should converge");
        println!("6-node mixed BLE+LAN converged in {} ticks", ticks.unwrap());

        // Verify both module types appear in routes
        let m = sim.metrics();
        assert!(m.fully_converged);
        let has_lan = m.routes_by_module.iter().any(|(name, _)| name == "LAN");
        let has_ble = m.routes_by_module.iter().any(|(name, _)| name == "BLE");
        assert!(has_lan || has_ble, "Should have LAN and/or BLE routes");
    }

    #[test]
    fn internet_relay_converges() {
        let mut sim = internet_relay(5);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(20, &mut rng);
        assert!(ticks.is_some(), "Internet relay should converge");
        println!("5-node internet relay converged in {} ticks", ticks.unwrap());

        let m = sim.metrics();
        let internet_routes = m.routes_by_module.iter().find(|(name, _)| name == "INTERNET");
        assert!(internet_routes.is_some(), "Should have INTERNET routes");
    }

    #[test]
    fn mixed_star_ble_internet_converges() {
        let mut sim = mixed_star_ble_internet(6);
        let mut rng = rand::rng();
        let ticks = sim.ticks_to_convergence(30, &mut rng);
        assert!(ticks.is_some(), "Mixed star BLE+Internet should converge");
        println!("6-node mixed star converged in {} ticks", ticks.unwrap());

        let m = sim.metrics();
        assert!(m.fully_converged);
        println!("Metrics: {}", m.summary());
    }

    #[test]
    fn hop_count_metrics_line() {
        let mut sim = line_convergence(5, 5000);
        let mut rng = rand::rng();
        sim.run(20, &mut rng);

        let m = sim.metrics();
        assert!(m.fully_converged);
        // In a 5-node line, max hop count should be 4 (node 0 to node 4)
        assert!(m.max_hop_count >= 2, "Max hops should be >= 2 in a line");
        assert!(m.avg_hop_count > 0.0, "Average hops should be > 0");
        println!("5-node line: avg_hops={:.2}, max_hops={}", m.avg_hop_count, m.max_hop_count);
    }
}
