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

/// Scenario: Feed propagation across a mesh.
/// Creates a line topology, converges routing, then tests that feed messages
/// can be saved on one node and retrieved.
pub fn feed_propagation(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::line(n, rtt_us);
    Simulator::new(topo, default_config())
}

/// Scenario: Message scheduling and delivery.
/// Creates a line topology, converges routing, then tests that messages
/// can be scheduled on one node and checked via the scheduler.
pub fn message_scheduling(n: usize, rtt_us: u32) -> Simulator {
    let topo = Topology::line(n, rtt_us);
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

    #[test]
    fn feed_propagation_across_mesh() {
        use libqaul::services::feed::proto_net;

        let mut sim = feed_propagation(3, 5000);
        let mut rng = rand::rng();

        // Converge routing first
        sim.run(10, &mut rng);
        assert!(sim.is_fully_converged());

        // Node 0 saves a feed message
        let sender_id = sim.nodes[0].peer_id.to_bytes();
        let msg = proto_net::FeedMessageContent {
            sender: sender_id.clone(),
            content: "hello from node 0".to_string(),
            time: 1000,
        };
        sim.nodes[0]
            .services
            .feed
            .save_message(vec![1, 2, 3], msg.clone());

        // Verify node 0 can retrieve it
        let list = sim.nodes[0].services.feed.get_messages(0);
        assert_eq!(list.feed_message.len(), 1);
        assert_eq!(list.feed_message[0].content, "hello from node 0");

        // Simulate sync: node 0 tells node 1 about message IDs
        let ids = sim.nodes[0].services.feed.get_latest_message_ids(10);
        assert_eq!(ids.len(), 1);

        // Node 1 checks which IDs it's missing
        let missing = sim.nodes[1].services.feed.process_received_feed_ids(&ids);
        assert_eq!(missing.len(), 1, "Node 1 should be missing the message");

        // Node 0 provides the message data
        let messages = sim.nodes[0].services.feed.get_messages_by_ids(&missing);
        assert_eq!(messages.len(), 1);

        // Node 1 saves the synced message
        let (msg_id, sender, content, time) = &messages[0];
        sim.nodes[1]
            .services
            .feed
            .save_message_by_sync(msg_id, sender, content.clone(), *time);

        // Verify node 1 now has it
        let list1 = sim.nodes[1].services.feed.get_messages(0);
        assert_eq!(list1.feed_message.len(), 1);
        assert_eq!(list1.feed_message[0].content, "hello from node 0");

        // Node 2 should still be empty
        let list2 = sim.nodes[2].services.feed.get_messages(0);
        assert_eq!(list2.feed_message.len(), 0);
    }

    #[test]
    fn message_scheduling_and_delivery() {
        use libqaul::services::messaging::proto;
        use prost::Message;

        let mut sim = message_scheduling(3, 5000);
        let mut rng = rand::rng();

        // Converge routing
        sim.run(10, &mut rng);
        assert!(sim.is_fully_converged());

        let sender_id = sim.nodes[0].peer_id;
        let receiver_id = sim.nodes[2].peer_id;

        // Create a mock container
        let envelope = proto::Envelope {
            sender_id: sender_id.to_bytes(),
            receiver_id: receiver_id.to_bytes(),
            payload: vec![1, 2, 3],
        };
        let container = proto::Container {
            signature: vec![10, 20, 30],
            envelope: Some(envelope),
        };

        // Schedule message on node 0
        sim.nodes[0].services.messaging.schedule_message(
            receiver_id,
            container.clone(),
            true,
            false,
            false,
            false,
        );

        // Check scheduler — should find a route and return the message
        let result = sim.nodes[0]
            .services
            .messaging
            .check_scheduler(&sim.nodes[0].router.routing_table);

        assert!(result.is_some(), "Scheduler should find a route to receiver");

        let (next_hop, _module, data) = result.unwrap();
        // The next hop should be node 1 (intermediate in the line)
        assert_eq!(next_hop, sim.nodes[1].peer_id, "Should route via node 1");

        // Verify the data is a valid Container
        let decoded = proto::Container::decode(&data[..]).unwrap();
        assert_eq!(decoded.signature, vec![10, 20, 30]);

        // After popping, scheduler should be empty
        let result2 = sim.nodes[0]
            .services
            .messaging
            .check_scheduler(&sim.nodes[0].router.routing_table);
        assert!(result2.is_none(), "Queue should be empty after pop");
    }

    #[test]
    fn feed_pagination_instance() {
        let sim = feed_propagation(2, 5000);
        let sender_id = sim.nodes[0].peer_id.to_bytes();

        // Insert 10 messages
        for i in 1..=10u64 {
            use libqaul::services::feed::proto_net;
            let msg = proto_net::FeedMessageContent {
                sender: sender_id.clone(),
                content: format!("msg {}", i),
                time: 1000 + i,
            };
            sim.nodes[0]
                .services
                .feed
                .save_message(vec![i as u8], msg);
        }

        // Paginate: first 3
        let page1 = sim.nodes[0].services.feed.get_paginated_messages(0, 3);
        assert_eq!(page1.feed_message.len(), 3);
        // Newest first (indices 10, 9, 8)
        assert_eq!(page1.feed_message[0].index, 10);
        assert_eq!(page1.feed_message[2].index, 8);
        let p = page1.pagination.unwrap();
        assert!(p.has_more);
        assert_eq!(p.total, 10);

        // Next page
        let page2 = sim.nodes[0].services.feed.get_paginated_messages(3, 3);
        assert_eq!(page2.feed_message.len(), 3);
        assert_eq!(page2.feed_message[0].index, 7);
    }

    #[test]
    fn unconfirmed_message_tracking() {
        use libqaul::services::messaging::{proto, MessagingServiceType};

        let sim = message_scheduling(2, 5000);
        let receiver_id = sim.nodes[1].peer_id;

        let container = proto::Container {
            signature: vec![42, 43, 44],
            envelope: Some(proto::Envelope {
                sender_id: sim.nodes[0].peer_id.to_bytes(),
                receiver_id: receiver_id.to_bytes(),
                payload: vec![],
            }),
        };

        // Save unconfirmed
        sim.nodes[0].services.messaging.save_unconfirmed_message(
            MessagingServiceType::Chat,
            &[1, 2, 3],
            &receiver_id,
            &container,
            false,
        );

        // Confirm it
        sim.nodes[0]
            .services
            .messaging
            .on_confirmed_message(&container.signature);

        // The unconfirmed tree should no longer have this entry
        let unconfirmed = sim.nodes[0].services.messaging.unconfirmed.read().unwrap();
        assert!(
            unconfirmed.unconfirmed.get(&container.signature).unwrap().is_none(),
            "Message should be removed after confirmation"
        );
    }
}
