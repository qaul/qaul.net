// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Test suite for `router_v2`, split to mirror the source layout.
//!
//! Shared fixture builders live in `test_utils.rs`; each module below covers
//! one concern and names the spec section it exercises.

mod apply_entry;
mod apply_mapping;
mod delegation;
mod delta_build;
mod forwarding;
mod gateway_role;
mod handle_index_dump;
mod handle_node_manifest;
mod handle_routing_update;
mod hop_sphere;
mod management;
mod mapping_sphere;
mod neighbour_transport;
mod next_hop;
mod on_neighbour_connect;
mod phase10_regressions;
mod propagation;
mod propagation_form;
mod rate_limits;
mod received;
mod relay;
mod self_and_neighbour_registration;
mod self_delegation;
mod sphere;
mod sweep;
mod translate;
