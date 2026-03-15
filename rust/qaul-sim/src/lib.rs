//! qaul-sim: Full-mesh simulation and benchmarking for qaul routing
//!
//! This crate creates multiple independent `RouterState` instances and
//! connects them via a simulated network topology, allowing testing of
//! routing convergence, message delivery, and performance without
//! any real networking.

pub mod metrics;
pub mod network;
pub mod scenarios;
pub mod simulator;
pub mod topology;
