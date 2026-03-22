//! Network topology representation
//!
//! Defines graph-based topologies of simulated nodes and links.
//! Each link has configurable latency, jitter, and packet loss.

use libqaul::connections::ConnectionModule;
use std::collections::HashMap;

/// A simulated link between two nodes.
#[derive(Debug, Clone)]
pub struct Link {
    /// Base round-trip time in microseconds.
    pub base_rtt_us: u32,
    /// Jitter standard deviation in microseconds.
    pub jitter_us: u32,
    /// Packet loss probability (0.0 = no loss, 1.0 = total loss).
    pub loss: f64,
    /// Whether the link is currently active.
    pub active: bool,
    /// Which connection module this link simulates.
    pub module: ConnectionModule,
}

impl Link {
    /// Create a new active link with the given RTT and no jitter/loss.
    /// Defaults to LAN module.
    pub fn new(base_rtt_us: u32) -> Self {
        Self {
            base_rtt_us,
            jitter_us: 0,
            loss: 0.0,
            active: true,
            module: ConnectionModule::Lan,
        }
    }

    /// Create a BLE link (typically higher latency, more loss).
    pub fn ble(base_rtt_us: u32) -> Self {
        Self {
            base_rtt_us,
            jitter_us: 0,
            loss: 0.0,
            active: true,
            module: ConnectionModule::Ble,
        }
    }

    /// Create an Internet link.
    pub fn internet(base_rtt_us: u32) -> Self {
        Self {
            base_rtt_us,
            jitter_us: 0,
            loss: 0.0,
            active: true,
            module: ConnectionModule::Internet,
        }
    }

    /// Set the connection module for this link.
    pub fn with_module(mut self, module: ConnectionModule) -> Self {
        self.module = module;
        self
    }

    /// Create a link with jitter.
    pub fn with_jitter(mut self, jitter_us: u32) -> Self {
        self.jitter_us = jitter_us;
        self
    }

    /// Create a link with packet loss.
    pub fn with_loss(mut self, loss: f64) -> Self {
        self.loss = loss;
        self
    }
}

/// Undirected graph of nodes and links.
/// Node indices are 0..n.
pub struct Topology {
    /// Number of nodes in the topology.
    pub node_count: usize,
    /// Edges: (node_a, node_b) -> Link.
    /// Stored with the smaller index first.
    pub links: HashMap<(usize, usize), Link>,
}

impl Topology {
    /// Create an empty topology with `n` nodes and no links.
    pub fn new(n: usize) -> Self {
        Self {
            node_count: n,
            links: HashMap::new(),
        }
    }

    /// Add a bidirectional link between two nodes.
    pub fn add_link(&mut self, a: usize, b: usize, link: Link) {
        let key = if a < b { (a, b) } else { (b, a) };
        self.links.insert(key, link);
    }

    /// Get the link between two nodes, if it exists and is active.
    pub fn get_link(&self, a: usize, b: usize) -> Option<&Link> {
        let key = if a < b { (a, b) } else { (b, a) };
        self.links.get(&key).filter(|l| l.active)
    }

    /// Get all active neighbours of a node.
    pub fn neighbours(&self, node: usize) -> Vec<(usize, &Link)> {
        let mut result = Vec::new();
        for (&(a, b), link) in &self.links {
            if !link.active {
                continue;
            }
            if a == node {
                result.push((b, link));
            } else if b == node {
                result.push((a, link));
            }
        }
        result
    }

    /// Set a link active or inactive (simulate failure/recovery).
    pub fn set_link_active(&mut self, a: usize, b: usize, active: bool) {
        let key = if a < b { (a, b) } else { (b, a) };
        if let Some(link) = self.links.get_mut(&key) {
            link.active = active;
        }
    }

    /// Create a line topology: 0-1-2-...-n with uniform RTT.
    pub fn line(n: usize, rtt_us: u32) -> Self {
        let mut topo = Self::new(n);
        for i in 0..n.saturating_sub(1) {
            topo.add_link(i, i + 1, Link::new(rtt_us));
        }
        topo
    }

    /// Create a ring topology: 0-1-2-...-n-0 with uniform RTT.
    pub fn ring(n: usize, rtt_us: u32) -> Self {
        let mut topo = Self::line(n, rtt_us);
        if n > 2 {
            topo.add_link(0, n - 1, Link::new(rtt_us));
        }
        topo
    }

    /// Create a grid topology (rows x cols) with uniform RTT.
    pub fn grid(rows: usize, cols: usize, rtt_us: u32) -> Self {
        let n = rows * cols;
        let mut topo = Self::new(n);
        for r in 0..rows {
            for c in 0..cols {
                let idx = r * cols + c;
                // right neighbour
                if c + 1 < cols {
                    topo.add_link(idx, idx + 1, Link::new(rtt_us));
                }
                // down neighbour
                if r + 1 < rows {
                    topo.add_link(idx, idx + cols, Link::new(rtt_us));
                }
            }
        }
        topo
    }

    /// Create a fully connected topology with uniform RTT.
    pub fn full_mesh(n: usize, rtt_us: u32) -> Self {
        let mut topo = Self::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                topo.add_link(i, j, Link::new(rtt_us));
            }
        }
        topo
    }

    /// Create a star topology with node 0 as center.
    pub fn star(n: usize, rtt_us: u32) -> Self {
        let mut topo = Self::new(n);
        for i in 1..n {
            topo.add_link(0, i, Link::new(rtt_us));
        }
        topo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_topology() {
        let topo = Topology::line(5, 1000);
        assert_eq!(topo.node_count, 5);
        assert_eq!(topo.links.len(), 4);
        assert!(topo.get_link(0, 1).is_some());
        assert!(topo.get_link(0, 2).is_none());
    }

    #[test]
    fn ring_topology() {
        let topo = Topology::ring(5, 1000);
        assert_eq!(topo.links.len(), 5);
        assert!(topo.get_link(0, 4).is_some());
    }

    #[test]
    fn grid_topology() {
        let topo = Topology::grid(3, 3, 1000);
        assert_eq!(topo.node_count, 9);
        // 3x3 grid has 2*3 + 3*2 = 12 edges
        assert_eq!(topo.links.len(), 12);
    }

    #[test]
    fn full_mesh_topology() {
        let topo = Topology::full_mesh(4, 1000);
        // C(4,2) = 6 edges
        assert_eq!(topo.links.len(), 6);
    }

    #[test]
    fn link_deactivation() {
        let mut topo = Topology::line(3, 1000);
        assert!(topo.get_link(0, 1).is_some());
        topo.set_link_active(0, 1, false);
        assert!(topo.get_link(0, 1).is_none());
        topo.set_link_active(0, 1, true);
        assert!(topo.get_link(0, 1).is_some());
    }

    #[test]
    fn neighbours_list() {
        let topo = Topology::star(4, 1000);
        let n = topo.neighbours(0);
        assert_eq!(n.len(), 3);
        let n = topo.neighbours(1);
        assert_eq!(n.len(), 1);
    }
}
