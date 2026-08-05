//! Test suite for `router_v2/mod.rs`. Split out of the module file to
//! keep the production surface readable. Shared fixture builders live in
//! `test_utils.rs`.

use super::*;

// ---------- Sphere ----------

mod sphere {
    use super::*;

    #[test]
    fn sphere_of_lan_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::Lan), Sphere::Local);
    }

    #[test]
    fn sphere_of_ble1m_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::Ble1m), Sphere::Local);
    }

    #[test]
    fn sphere_of_ble_coded_is_local() {
        assert_eq!(Sphere::of(ConnectionModule::BleCoded), Sphere::Local);
    }

    #[test]
    fn sphere_of_internet_is_internet() {
        assert_eq!(Sphere::of(ConnectionModule::Internet), Sphere::Internet);
    }

    #[test]
    fn sphere_of_self_is_local() {
        // ConnectionModule::Local refers to this node itself, which is
        // part of its own Local sphere by definition
        assert_eq!(Sphere::of(ConnectionModule::Local), Sphere::Local);
    }

    #[test]
    fn sphere_of_none_currently_falls_through_to_local() {
        assert_eq!(Sphere::of(ConnectionModule::None), Sphere::Local);
    }
}

// ---------- next_hop_for_user ----------

mod next_hop {
    use super::*;
    use crate::router_v2::{
        index::Space,
        seq::SeqNum,
        table::{Node, RoutingEntry, TargetRef},
        test_utils::*,
    };
    use std::sync::Weak;

    fn make_entry(
        target: TargetRef,
        next_hop: u16,
        metric: u16,
        transport: ConnectionModule,
    ) -> Arc<RwLock<RoutingEntry>> {
        Arc::new(RwLock::new(RoutingEntry {
            target_index: 0,
            target,
            seq_num: SeqNum::from(0u16),
            metric,
            next_hop,
            transport,
            last_update: 0,
            hop_count: 0,
            local_only: false,
        }))
    }

    #[test]
    fn unknown_user_returns_none() {
        let (state, _rx) = fresh_state();
        assert_eq!(state.next_hop_for_user([99; 8]), None);
    }

    #[test]
    fn known_user_with_no_routing_data_returns_none() {
        let (state, _rx) = fresh_state();
        install_user(&state, [1; 8], 0);
        assert_eq!(state.next_hop_for_user([1; 8]), None);
    }

    /// Step 2: a direct routing entry whose next_hop resolves through the
    /// dictionary should produce that hop's node id and the entry's transport.
    #[test]
    fn direct_routing_entry_resolves_next_hop_and_transport() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let neighbour_id = [9; 8];
        let neighbour_idx = 100;
        bind_own_dict(&state, Space::Node, neighbour_idx, neighbour_id);

        let entry = make_entry(
            TargetRef::User(user.clone()),
            neighbour_idx,
            42,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, 5, entry.clone());
        user.write().unwrap().routing_entry = Some(Arc::downgrade(&entry));

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((neighbour_id, ConnectionModule::Lan)),
        );
    }

    /// Step 3: no direct entry; two delegation gateways; the gateway with
    /// the lowest metric wins, and its routing entry's next_hop / transport
    /// determine the result.
    #[test]
    fn gateway_fallback_picks_lowest_metric() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let g_hi = install_node(&state, [10; 8], 0, true);
        let g_lo = install_node(&state, [20; 8], 0, true);

        bind_own_dict(&state, Space::Node, 50, [10; 8]);
        bind_own_dict(&state, Space::Node, 60, [20; 8]);

        let nbr_hi = [11; 8];
        let nbr_lo = [21; 8];
        bind_own_dict(&state, Space::Node, 101, nbr_hi);
        bind_own_dict(&state, Space::Node, 102, nbr_lo);

        let e_hi = make_entry(
            TargetRef::Node(g_hi.clone()),
            101,
            30,
            ConnectionModule::Lan,
        );
        let e_lo = make_entry(
            TargetRef::Node(g_lo.clone()),
            102,
            10,
            ConnectionModule::Internet,
        );
        {
            let mut rt = state.routing_table.write().unwrap();
            rt.set(Space::Node, 50, e_hi);
            rt.set(Space::Node, 60, e_lo);
        }

        user.write().unwrap().delegation_gateways =
            vec![Arc::downgrade(&g_hi), Arc::downgrade(&g_lo)];

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((nbr_lo, ConnectionModule::Internet)),
        );
    }

    /// A direct routing entry must be preferred over a delegation gateway,
    /// even when the gateway has a lower metric.
    #[test]
    fn direct_entry_preferred_over_lower_metric_gateway() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let direct_nbr = [50; 8];
        bind_own_dict(&state, Space::Node, 200, direct_nbr);
        let direct = make_entry(
            TargetRef::User(user.clone()),
            200,
            100,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, 5, direct.clone());

        let gw = install_node(&state, [10; 8], 0, true);
        bind_own_dict(&state, Space::Node, 50, [10; 8]);
        let gw_nbr = [11; 8];
        bind_own_dict(&state, Space::Node, 101, gw_nbr);
        let gw_entry = make_entry(
            TargetRef::Node(gw.clone()),
            101,
            5,
            ConnectionModule::Internet,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, gw_entry);

        {
            let mut u = user.write().unwrap();
            u.routing_entry = Some(Arc::downgrade(&direct));
            u.delegation_gateways.push(Arc::downgrade(&gw));
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((direct_nbr, ConnectionModule::Lan)),
        );
    }

    #[test]
    fn dangling_direct_entry_falls_through_to_gateway() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let orphan = make_entry(TargetRef::User(user.clone()), 0, 100, ConnectionModule::Lan);
        let dangling: Weak<RwLock<RoutingEntry>> = Arc::downgrade(&orphan);
        drop(orphan);

        let gw = install_node(&state, [10; 8], 0, true);
        bind_own_dict(&state, Space::Node, 50, [10; 8]);
        let gw_nbr = [11; 8];
        bind_own_dict(&state, Space::Node, 101, gw_nbr);
        let gw_entry = make_entry(
            TargetRef::Node(gw.clone()),
            101,
            5,
            ConnectionModule::Internet,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, gw_entry);

        {
            let mut u = user.write().unwrap();
            u.routing_entry = Some(dangling);
            u.delegation_gateways.push(Arc::downgrade(&gw));
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((gw_nbr, ConnectionModule::Internet)),
        );
    }

    #[test]
    fn dangling_gateway_is_skipped() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let live = install_node(&state, [10; 8], 0, true);
        bind_own_dict(&state, Space::Node, 50, [10; 8]);
        let live_nbr = [11; 8];
        bind_own_dict(&state, Space::Node, 101, live_nbr);
        let live_entry = make_entry(
            TargetRef::Node(live.clone()),
            101,
            30,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, live_entry);

        // Dangling gateway: build a Node, take a Weak, drop the strong.
        let orphan = Arc::new(RwLock::new(Node {
            id: [20; 8],
            public_key: Some(fresh_multikey()),
            manifest_version: 0,
            advertised_version: 0,
            is_gateway: true,
            delegated_users: Vec::new(),
            manifest_signature: None,
            retained_chunks: None,
            learn_sphere: None,
            manifest_log: crate::router_v2::manifest::ManifestLog::default(),
        }));
        let dangling = Arc::downgrade(&orphan);
        drop(orphan);

        {
            let mut u = user.write().unwrap();
            u.delegation_gateways = vec![dangling, Arc::downgrade(&live)];
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((live_nbr, ConnectionModule::Lan)),
        );
    }

    #[test]
    fn gateway_with_no_routing_entry_is_skipped() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let unreachable = install_node(&state, [20; 8], 0, true);
        bind_own_dict(&state, Space::Node, 60, [20; 8]);

        let reachable = install_node(&state, [10; 8], 0, true);
        bind_own_dict(&state, Space::Node, 50, [10; 8]);
        let r_nbr = [11; 8];
        bind_own_dict(&state, Space::Node, 101, r_nbr);
        let r_entry = make_entry(
            TargetRef::Node(reachable.clone()),
            101,
            5,
            ConnectionModule::Lan,
        );
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::Node, 50, r_entry);

        {
            let mut u = user.write().unwrap();
            u.delegation_gateways = vec![Arc::downgrade(&unreachable), Arc::downgrade(&reachable)];
        }

        assert_eq!(
            state.next_hop_for_user([1; 8]),
            Some((r_nbr, ConnectionModule::Lan)),
        );
    }

    #[test]
    fn next_hop_node_id_resolves_bound_indices_and_misses_unbound() {
        let (state, _rx) = fresh_state();
        bind_own_dict(&state, Space::Node, 77, [7; 8]);
        assert_eq!(state.next_hop_node_id(77), Some([7; 8]));
        assert_eq!(state.next_hop_node_id(78), None);
    }
}

// ---------- sweep_expired ----------

mod sweep {
    use super::*;
    use crate::router_v2::{
        index::Space,
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
    };
    use std::sync::Weak;

    /// Installs a routing entry at `(space, idx)`, binds the dictionary,
    /// and returns a Weak to the entry so tests can verify cycle
    /// discipline after sweep.
    fn install_entry(
        state: &RouterV2State,
        space: Space,
        idx: u16,
        target_id: [u8; 8],
        target: TargetRef,
        last_update: u64,
    ) -> Weak<RwLock<RoutingEntry>> {
        let arc = Arc::new(RwLock::new(RoutingEntry {
            target_index: idx,
            target,
            seq_num: SeqNum::from(0u16),
            metric: 0,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update,
            hop_count: 0,
            local_only: false,
        }));
        let weak = Arc::downgrade(&arc);
        state.routing_table.write().unwrap().set(space, idx, arc);
        bind_own_dict(state, space, idx, target_id);
        weak
    }

    fn expiry_ms(state: &RouterV2State) -> u64 {
        state.options.route_expiry_ms
    }

    #[test]
    fn entry_past_threshold_is_removed() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        state.sweep_expired(now);

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 5)
            .is_none());
    }

    #[test]
    fn entry_within_threshold_is_kept() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) + 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        state.sweep_expired(now);

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 5)
            .is_some());
    }

    /// At exactly `last_update + expiry == now`, the strict `<` comparison
    /// keeps the entry. Pins the operator against an accidental `<=`.
    #[test]
    fn entry_at_exact_boundary_is_kept() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state);
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        state.sweep_expired(now);

        assert!(
            state
                .routing_table
                .read()
                .unwrap()
                .get(Space::User, 5)
                .is_some(),
            "entry exactly at the threshold must survive (strict `<`)",
        );
    }

    #[test]
    fn expired_entry_unbinds_the_dictionary() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        assert_eq!(state.user_dict.read().unwrap().id_of(5), Some([1; 8]));

        state.sweep_expired(now);

        assert_eq!(state.user_dict.read().unwrap().id_of(5), None);
        assert_eq!(state.user_dict.read().unwrap().idx_of(&[1; 8]), None);
    }

    #[test]
    fn expired_entry_pushes_idx_into_allocator_cooldown() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user),
            last_update,
        );

        assert!(!state.users_allocator.read().unwrap().idx_in_cooldown(5));

        state.sweep_expired(now);

        assert!(
            state.users_allocator.read().unwrap().idx_in_cooldown(5),
            "released idx must enter cooldown so the allocator doesn't rebind it immediately",
        );
    }

    /// Cycle discipline (spec A.3): once the table drops its Arc, the
    /// User's back-edge Weak must resolve to None.
    #[test]
    fn expired_entry_makes_user_weak_routing_entry_dangle() {
        let (state, _rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        let weak = install_entry(
            &state,
            Space::User,
            5,
            [1; 8],
            TargetRef::User(user.clone()),
            last_update,
        );
        user.write().unwrap().routing_entry = Some(weak.clone());

        assert!(weak.upgrade().is_some(), "weak must upgrade before sweep");

        state.sweep_expired(now);

        assert!(
            weak.upgrade().is_none(),
            "weak must dangle after sweep drops the table's Arc",
        );
        assert!(user.read().unwrap().routing_entry.is_some());
    }

    #[test]
    fn node_space_expiry_is_independent_from_user_space() {
        let (state, _rx) = fresh_state();
        let node = install_node(&state, [9; 8], 0, false);
        let now: u64 = 100_000;
        let last_update = now - expiry_ms(&state) - 1;
        install_entry(
            &state,
            Space::Node,
            7,
            [9; 8],
            TargetRef::Node(node),
            last_update,
        );

        let user = install_user(&state, [1; 8], 0);
        install_entry(&state, Space::User, 3, [1; 8], TargetRef::User(user), now);

        state.sweep_expired(now);

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::Node, 7)
            .is_none());
        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 3)
            .is_some());
        assert!(state.node_allocator.read().unwrap().idx_in_cooldown(7));
        assert!(!state.users_allocator.read().unwrap().idx_in_cooldown(3));
    }

    #[test]
    fn mixed_entries_only_expired_are_removed() {
        let (state, _rx) = fresh_state();
        let now: u64 = 100_000;

        let old_user = install_user(&state, [1; 8], 0);
        let fresh_user = install_user(&state, [2; 8], 0);

        install_entry(
            &state,
            Space::User,
            10,
            [1; 8],
            TargetRef::User(old_user),
            now - expiry_ms(&state) - 1,
        );
        install_entry(
            &state,
            Space::User,
            11,
            [2; 8],
            TargetRef::User(fresh_user),
            now,
        );

        state.sweep_expired(now);

        let rt = state.routing_table.read().unwrap();
        assert!(rt.get(Space::User, 10).is_none(), "stale entry removed");
        assert!(rt.get(Space::User, 11).is_some(), "fresh entry untouched");
    }

    #[test]
    fn sweep_on_empty_state_is_a_noop() {
        let (state, _rx) = fresh_state();
        state.sweep_expired(0);
        state.sweep_expired(u64::MAX);
    }
}

// ---------- translate_incoming + pending_introductions ----------

mod translate {
    use super::*;
    use crate::router_v2::{index::Space, test_utils::*};

    #[test]
    fn translate_incoming_unknown_neighbour_returns_unknown_mapping() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();
        let err = state.translate_incoming(peer, Space::User, 5).unwrap_err();
        assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
    }

    #[test]
    fn translate_incoming_known_neighbour_unknown_idx_returns_unknown_mapping() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let err = state.translate_incoming(peer, Space::User, 5).unwrap_err();
        assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
    }

    /// If our own dict already has a binding for the ID, return the
    /// existing own_idx; do not allocate, do not mark the tracker.
    #[test]
    fn translate_incoming_existing_own_binding_returns_existing_idx() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [7; 8];

        bind_mirror(&state, peer, Space::User, 5, id);
        state.user_dict.write().unwrap().bind(99, id);

        let got = state.translate_incoming(peer, Space::User, 5).unwrap();
        assert_eq!(got, 99);

        let pending = state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::User);
        assert!(
            pending.is_empty(),
            "existing-binding path must not touch the tracker"
        );
    }

    #[test]
    fn translate_incoming_fresh_allocates_binds_and_marks_tracker() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [11; 8];
        bind_mirror(&state, peer, Space::User, 5, id);

        let allocated_idx = state.translate_incoming(peer, Space::User, 5).unwrap();

        let dict = state.user_dict.read().unwrap();
        assert_eq!(dict.idx_of(&id), Some(allocated_idx));
        assert_eq!(dict.id_of(allocated_idx), Some(id));
        drop(dict);

        let pending = state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::User);
        assert!(pending.contains(&allocated_idx));
    }

    #[test]
    fn translate_incoming_is_idempotent_for_same_id() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [13; 8];
        bind_mirror(&state, peer, Space::User, 5, id);

        let first = state.translate_incoming(peer, Space::User, 5).unwrap();
        let second = state.translate_incoming(peer, Space::User, 5).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn translate_incoming_spaces_are_independent() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let user_id = [21; 8];
        let node_id = [22; 8];

        bind_mirror(&state, peer, Space::User, 5, user_id);
        bind_mirror(&state, peer, Space::Node, 5, node_id);

        let user_idx = state.translate_incoming(peer, Space::User, 5).unwrap();
        let node_idx = state.translate_incoming(peer, Space::Node, 5).unwrap();

        assert_eq!(
            state.user_dict.read().unwrap().id_of(user_idx),
            Some(user_id)
        );
        assert_eq!(
            state.node_dict.read().unwrap().id_of(node_idx),
            Some(node_id)
        );
        assert_eq!(state.node_dict.read().unwrap().idx_of(&user_id), None);
        assert_eq!(state.user_dict.read().unwrap().idx_of(&node_id), None);
    }

    // ---------- pending_introductions ----------

    #[test]
    fn pending_introductions_empty_when_no_marks() {
        let (state, _rx) = fresh_state();
        assert!(state.pending_introductions(Space::User).is_empty());
        assert!(state.pending_introductions(Space::Node).is_empty());
    }

    #[test]
    fn pending_introductions_returns_marked_user_with_correct_version() {
        let (state, _rx) = fresh_state();
        let id = [3; 8];
        install_user(&state, id, 42);
        state.user_dict.write().unwrap().bind(7, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 7);

        let out = state.pending_introductions(Space::User);
        assert_eq!(out, vec![(7, id, 42)]);
    }

    #[test]
    fn pending_introductions_returns_marked_node_with_correct_version() {
        let (state, _rx) = fresh_state();
        let id = [4; 8];
        install_node(&state, id, 99, false);
        state.node_dict.write().unwrap().bind(8, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::Node, 8);

        let out = state.pending_introductions(Space::Node);
        assert_eq!(out, vec![(8, id, 99)]);
    }

    #[test]
    fn pending_introductions_drains_only_requested_space() {
        let (state, _rx) = fresh_state();

        let user_id = [1; 8];
        install_user(&state, user_id, 5);
        state.user_dict.write().unwrap().bind(10, user_id);

        let node_id = [2; 8];
        install_node(&state, node_id, 6, false);
        state.node_dict.write().unwrap().bind(20, node_id);

        {
            let mut t = state.reintroduction_tracker.write().unwrap();
            t.mark_first_time(Space::User, 10);
            t.mark_first_time(Space::Node, 20);
        }

        let users = state.pending_introductions(Space::User);
        assert_eq!(users, vec![(10, user_id, 5)]);

        let nodes = state.pending_introductions(Space::Node);
        assert_eq!(nodes, vec![(20, node_id, 6)]);
    }

    #[test]
    fn pending_introductions_second_call_returns_empty_after_drain() {
        let (state, _rx) = fresh_state();
        let id = [9; 8];
        install_user(&state, id, 1);
        state.user_dict.write().unwrap().bind(3, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 3);

        let first = state.pending_introductions(Space::User);
        assert!(!first.is_empty(), "first call should drain the mark");

        let second = state.pending_introductions(Space::User);
        assert!(second.is_empty(), "second call should be empty after drain");
    }

    /// Phase 8's delta encoder requires ascending idx order.
    #[test]
    fn pending_introductions_results_sorted_by_index() {
        let (state, _rx) = fresh_state();
        let ids: Vec<[u8; 8]> = (1..=5).map(|i| [i as u8; 8]).collect();
        let idxs = [50u16, 10, 200, 30, 80];

        for (i, idx) in idxs.iter().enumerate() {
            install_user(&state, ids[i], i as u32);
            state.user_dict.write().unwrap().bind(*idx, ids[i]);
            state
                .reintroduction_tracker
                .write()
                .unwrap()
                .mark_first_time(Space::User, *idx);
        }

        let out = state.pending_introductions(Space::User);
        let returned_idxs: Vec<u16> = out.iter().map(|(idx, _, _)| *idx).collect();
        let mut expected = idxs.to_vec();
        expected.sort();
        assert_eq!(returned_idxs, expected);
    }

    #[test]
    fn pending_introductions_skips_orphan_with_no_dict_binding() {
        let (state, _rx) = fresh_state();
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 42);

        let out = state.pending_introductions(Space::User);
        assert!(
            out.is_empty(),
            "orphan mark with no dict binding must be skipped"
        );
    }

    #[test]
    fn pending_introductions_skips_orphan_with_no_record() {
        let (state, _rx) = fresh_state();
        let id = [77; 8];
        state.user_dict.write().unwrap().bind(42, id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 42);

        let out = state.pending_introductions(Space::User);
        assert!(out.is_empty(), "missing user record must be skipped");
    }

    #[test]
    fn pending_introductions_mixed_healthy_and_orphan() {
        let (state, _rx) = fresh_state();

        let good_id = [1; 8];
        install_user(&state, good_id, 7);
        state.user_dict.write().unwrap().bind(10, good_id);

        {
            let mut t = state.reintroduction_tracker.write().unwrap();
            t.mark_first_time(Space::User, 10);
            t.mark_first_time(Space::User, 99); // orphan
        }

        let out = state.pending_introductions(Space::User);
        assert_eq!(out, vec![(10, good_id, 7)]);
    }
}

// ---------- apply_mapping ----------

mod apply_mapping {
    use super::*;
    use crate::router_v2::{
        codec::messages::Mapping,
        index::Space,
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
    };

    #[test]
    fn apply_mapping_unknown_neighbour_is_noop() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();

        let result = state.apply_mapping(
            peer,
            Space::User,
            Mapping {
                abs_idx: 5,
                target_id: [1; 8],
                version: 42,
            },
        );

        assert!(result.is_ok());
        assert_eq!(state.users.read().unwrap().len(), 0);
        assert!(state.mirrors.read().unwrap().is_empty());
    }

    #[test]
    fn apply_mapping_fresh_user_creates_stub_and_binds_mirror() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping {
                    abs_idx: 5,
                    target_id: [1; 8],
                    version: 42,
                },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some([1; 8]));
        drop(mirrors);

        let users = state.users.read().unwrap();
        let user_arc = users.get(&[1; 8]).unwrap();
        let user = user_arc.read().unwrap();
        assert_eq!(user.id, [1; 8]);
        assert_eq!(user.profile_version, 42);
        assert!(user.public_key.is_none(), "stub must not fabricate a key");
    }

    #[test]
    fn apply_mapping_fresh_node_creates_stub_and_binds_mirror() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        state
            .apply_mapping(
                peer,
                Space::Node,
                Mapping {
                    abs_idx: 5,
                    target_id: [2; 8],
                    version: 99,
                },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().nodes.id_of(5), Some([2; 8]));
        drop(mirrors);

        let nodes = state.nodes.read().unwrap();
        let node = nodes.get(&[2; 8]).unwrap();
        let n = node.read().unwrap();
        // Under the pull-based model (§10.8), a node mapping carries an
        // *advertisement* of the origin's manifest_version, not the
        // committed value. Stub nodes have manifest_version=0 (no
        // committed manifest yet); the mapping's version writes to
        // advertised_version and later drives the pull trigger.
        assert_eq!(n.manifest_version, 0, "stub node has no committed manifest");
        assert_eq!(
            n.advertised_version, 99,
            "mapping's version → advertised_version"
        );
        assert!(!n.is_gateway, "stub node is not a gateway by default");
        assert!(n.public_key.is_none());
    }

    #[test]
    fn apply_mapping_same_id_updates_version_only() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [3; 8];

        bind_mirror(&state, peer, Space::User, 5, id);
        install_user(&state, id, 10);

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping {
                    abs_idx: 5,
                    target_id: id,
                    version: 20,
                },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(id));
        drop(mirrors);

        let users = state.users.read().unwrap();
        assert_eq!(users.get(&id).unwrap().read().unwrap().profile_version, 20);
    }

    /// The critical §8.7-step-2 case: mirror already has abs_idx bound to
    /// OLD; applying NEW must clear old routing entry, release own_idx to
    /// cooldown, unbind own dict, then bind new mapping.
    #[test]
    fn apply_mapping_rebind_clears_old_routing_state() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        let old_id = [10; 8];
        let new_id = [20; 8];
        let own_idx: u16 = 7;

        bind_mirror(&state, peer, Space::User, 5, old_id);
        let old_user = install_user(&state, old_id, 1);
        state.user_dict.write().unwrap().bind(own_idx, old_id);

        let entry = Arc::new(RwLock::new(RoutingEntry {
            target_index: own_idx,
            target: TargetRef::User(old_user.clone()),
            seq_num: SeqNum::from(0u16),
            metric: 5,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update: 0,
            hop_count: 1,
            local_only: false,
        }));
        let entry_weak = Arc::downgrade(&entry);
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, own_idx, entry);
        old_user.write().unwrap().routing_entry = Some(entry_weak.clone());

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping {
                    abs_idx: 5,
                    target_id: new_id,
                    version: 1,
                },
            )
            .unwrap();

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, own_idx)
            .is_none());
        assert!(
            entry_weak.upgrade().is_none(),
            "old routing entry Arc must be dropped"
        );

        assert_eq!(state.user_dict.read().unwrap().idx_of(&old_id), None);
        assert_eq!(state.user_dict.read().unwrap().id_of(own_idx), None);

        assert!(state
            .users_allocator
            .read()
            .unwrap()
            .idx_in_cooldown(own_idx));

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(new_id));
        drop(mirrors);

        assert!(state.users.read().unwrap().get(&new_id).is_some());
    }

    #[test]
    fn apply_mapping_incoming_version_equal_is_noop() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [4; 8];
        install_user(&state, id, 42);

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping {
                    abs_idx: 5,
                    target_id: id,
                    version: 42,
                },
            )
            .unwrap();

        assert_eq!(
            state
                .users
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .read()
                .unwrap()
                .profile_version,
            42,
        );
    }

    #[test]
    fn apply_mapping_incoming_version_older_preserves_stored() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [5; 8];
        install_user(&state, id, 100);

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping {
                    abs_idx: 5,
                    target_id: id,
                    version: 50,
                },
            )
            .unwrap();

        assert_eq!(
            state
                .users
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .read()
                .unwrap()
                .profile_version,
            100,
            "stale-echo path must NOT overwrite the fresher stored version",
        );
    }

    /// A fresher advertised version updates Node.advertised_version
    /// (the hint), not the committed manifest_version. The committed
    /// value only advances when a verified manifest lands (§10.8).
    #[test]
    fn apply_mapping_incoming_version_fresher_updates_advertised_only() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let id = [6; 8];
        // install_node sets manifest_version=5 (committed); advertised_version=0.
        install_node(&state, id, 5, false);

        state
            .apply_mapping(
                peer,
                Space::Node,
                Mapping {
                    abs_idx: 5,
                    target_id: id,
                    version: 12,
                },
            )
            .unwrap();

        let nodes = state.nodes.read().unwrap();
        let node = nodes.get(&id).unwrap();
        let n = node.read().unwrap();
        // Committed value stays at 5 — we haven't verified a manifest at 12.
        assert_eq!(
            n.manifest_version, 5,
            "committed manifest_version must not change from a mapping"
        );
        // The hint updates so the pull trigger can compare.
        assert_eq!(
            n.advertised_version, 12,
            "advertised_version records the incoming hint"
        );
    }

    #[test]
    fn apply_mapping_user_and_node_spaces_are_independent() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);
        let user_id = [11; 8];
        let node_id = [22; 8];

        state
            .apply_mapping(
                peer,
                Space::User,
                Mapping {
                    abs_idx: 5,
                    target_id: user_id,
                    version: 1,
                },
            )
            .unwrap();
        state
            .apply_mapping(
                peer,
                Space::Node,
                Mapping {
                    abs_idx: 5,
                    target_id: node_id,
                    version: 1,
                },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        let nm = mirrors.get(&peer).unwrap();
        assert_eq!(nm.users.id_of(5), Some(user_id));
        assert_eq!(nm.nodes.id_of(5), Some(node_id));
        drop(mirrors);

        assert!(state.users.read().unwrap().get(&user_id).is_some());
        assert!(state.users.read().unwrap().get(&node_id).is_none());
        assert!(state.nodes.read().unwrap().get(&node_id).is_some());
        assert!(state.nodes.read().unwrap().get(&user_id).is_none());
    }
}

// ---------- apply_entry ----------

mod apply_entry {
    use super::*;
    use crate::router_v2::{
        codec::messages::{NodeEntry, UserEntry},
        index::Space,
        receive::ReceiveCtx,
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
    };
    use libp2p::PeerId;

    /// Build a ReceiveCtx with defaults every test uses (Lan transport,
    /// no RSSI). Callers vary neighbour and `now`.
    fn default_ctx(peer: PeerId, now: u64) -> ReceiveCtx {
        ReceiveCtx {
            neighbour: peer,
            transport: ConnectionModule::Lan,
            rssi_dbm: None,
            now,
        }
    }

    const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
    const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;

    /// Wires everything a user-space `apply_entry` call needs:
    /// - a neighbour with a distinct node_id, added to mirrors
    /// - that node_id bound in node_dict (so next_hop resolution succeeds)
    /// - the incoming `abs_idx` bound in the neighbour's mirror to `target_id`
    /// - `target_id` pre-bound in our own user_dict at `own_idx`
    ///   (so translate_incoming hits the existing-binding fast path)
    /// - a stub User record for `target_id`
    fn setup_user_target(
        state: &RouterV2State,
        abs_idx: u16,
        own_idx: u16,
        target_id: [u8; 8],
    ) -> (PeerId, Arc<RwLock<crate::router_v2::table::User>>) {
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        bind_mirror(state, peer, Space::User, abs_idx, target_id);
        bind_own_dict(state, Space::User, own_idx, target_id);
        bind_own_dict(
            state,
            Space::Node,
            NEIGHBOUR_IDX_IN_NODE_DICT,
            NEIGHBOUR_NODE_ID,
        );
        let user = install_user(state, target_id, 0);
        (peer, user)
    }

    fn setup_node_target(
        state: &RouterV2State,
        abs_idx: u16,
        own_idx: u16,
        target_id: [u8; 8],
    ) -> (PeerId, Arc<RwLock<crate::router_v2::table::Node>>) {
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        bind_mirror(state, peer, Space::Node, abs_idx, target_id);
        bind_own_dict(state, Space::Node, own_idx, target_id);
        bind_own_dict(
            state,
            Space::Node,
            NEIGHBOUR_IDX_IN_NODE_DICT,
            NEIGHBOUR_NODE_ID,
        );
        let node = install_node(state, target_id, 0, false);
        (peer, node)
    }

    /// Preload a routing-table slot with a stored entry for §7.2
    /// comparison tests.
    fn preload_entry(
        state: &RouterV2State,
        space: Space,
        own_idx: u16,
        target: TargetRef,
        seq: u16,
        metric: u16,
        local_only: bool,
    ) {
        let entry = Arc::new(RwLock::new(RoutingEntry {
            target_index: own_idx,
            target,
            seq_num: SeqNum::from(seq),
            metric,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update: 0,
            hop_count: 0,
            local_only,
        }));
        state
            .routing_table
            .write()
            .unwrap()
            .set(space, own_idx, entry);
    }

    fn wire_user_entry(
        abs_idx: u16,
        seq: u16,
        metric: u16,
        hop_count: u8,
        local_only: bool,
    ) -> UserEntry {
        UserEntry {
            abs_idx,
            seq,
            metric,
            hop_count,
            local_only,
        }
    }

    fn wire_node_entry(
        abs_idx: u16,
        seq: u16,
        metric: u16,
        hop_count: u8,
        local_only: bool,
        manifest_version: u32,
    ) -> NodeEntry {
        NodeEntry {
            abs_idx,
            seq,
            metric,
            hop_count,
            local_only,
            manifest_version,
        }
    }

    // ---------- TTL / drops ----------

    #[test]
    fn ttl_drop_when_incoming_hop_count_is_63() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, _) = setup_user_target(&state, 5, 42, target_id);

        state
            .apply_user_entry(
                &default_ctx(peer, 1_000),
                wire_user_entry(5, 1, 10, 63, false),
            )
            .unwrap();

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .is_none());
    }

    #[test]
    fn hop_count_62_is_accepted_and_stored_as_63() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, _) = setup_user_target(&state, 5, 42, target_id);

        state
            .apply_user_entry(
                &default_ctx(peer, 1_000),
                wire_user_entry(5, 1, 10, 62, false),
            )
            .unwrap();

        let rt = state.routing_table.read().unwrap();
        let stored = rt.get(Space::User, 42).unwrap();
        assert_eq!(stored.read().unwrap().hop_count, 63);
    }

    #[test]
    fn unknown_mapping_drops_silently() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        // Neighbour exists but has no mirror binding at abs_idx 5.
        bind_own_dict(
            &state,
            Space::Node,
            NEIGHBOUR_IDX_IN_NODE_DICT,
            NEIGHBOUR_NODE_ID,
        );

        state
            .apply_user_entry(
                &default_ctx(peer, 1_000),
                wire_user_entry(5, 1, 10, 1, false),
            )
            .unwrap();

        assert!(state.users.read().unwrap().len() == 0);
        assert!(state
            .routing_table
            .read()
            .unwrap()
            .user_entries
            .iter()
            .all(|s| s.is_none()));
    }

    /// The mapping section is required to create a stub User before entries
    /// reference the target. If it hasn't, the entry must be dropped rather
    /// than trigger a fabricated record.
    #[test]
    fn missing_user_target_record_drops() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();
        let target_id = [1; 8];
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        bind_mirror(&state, peer, Space::User, 5, target_id);
        bind_own_dict(&state, Space::User, 42, target_id);
        bind_own_dict(
            &state,
            Space::Node,
            NEIGHBOUR_IDX_IN_NODE_DICT,
            NEIGHBOUR_NODE_ID,
        );
        // NOTE: no install_user — the record is missing.

        state
            .apply_user_entry(
                &default_ctx(peer, 1_000),
                wire_user_entry(5, 1, 10, 1, false),
            )
            .unwrap();

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .is_none());
    }

    #[test]
    fn neighbour_node_id_not_in_node_dict_drops() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();
        let target_id = [1; 8];
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        bind_mirror(&state, peer, Space::User, 5, target_id);
        bind_own_dict(&state, Space::User, 42, target_id);
        install_user(&state, target_id, 0);
        // NOTE: no bind_own_dict for NEIGHBOUR_NODE_ID — step 7 must fail.

        state
            .apply_user_entry(
                &default_ctx(peer, 1_000),
                wire_user_entry(5, 1, 10, 1, false),
            )
            .unwrap();

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .is_none());
    }

    // ---------- empty-slot accept ----------

    /// Full happy path for a user-space entry into an empty slot: verifies
    /// every RoutingEntry field, that the User's Weak back-edge is
    /// attached, and that metric composition + hop-count increment applied.
    #[test]
    fn empty_slot_accept_stores_entry_and_attaches_user_back_edge() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);

        state
            .apply_user_entry(
                &default_ctx(peer, 1_234),
                wire_user_entry(5, 7, 10, 2, false),
            )
            .unwrap();

        let rt = state.routing_table.read().unwrap();
        let stored_arc = rt.get(Space::User, 42).expect("slot must be occupied");
        let stored = stored_arc.read().unwrap();

        assert_eq!(stored.target_index, 42);
        assert_eq!(stored.seq_num, SeqNum::from(7u16));
        // Lan weight is 10, no BLE RSSI → penalty 0 → metric = 10 + 10 = 20.
        assert_eq!(stored.metric, 20);
        assert_eq!(stored.next_hop, NEIGHBOUR_IDX_IN_NODE_DICT);
        assert_eq!(stored.transport, ConnectionModule::Lan);
        assert_eq!(stored.last_update, 1_234);
        assert_eq!(stored.hop_count, 3);
        assert!(!stored.local_only);

        // Weak back-edge on the User points at the stored entry.
        let weak = user.read().unwrap().routing_entry.clone().unwrap();
        let upgraded = weak.upgrade().unwrap();
        assert!(Arc::ptr_eq(&upgraded, &stored_arc));
    }

    #[test]
    fn empty_slot_accept_for_node_target_stores_entry() {
        let (state, _rx) = fresh_state();
        let target_id = [2; 8];
        let (peer, _) = setup_node_target(&state, 5, 42, target_id);

        state
            .apply_node_entry(
                &default_ctx(peer, 500),
                wire_node_entry(5, 1, 10, 0, false, 0),
            )
            .unwrap();

        let rt = state.routing_table.read().unwrap();
        let stored = rt.get(Space::Node, 42).expect("slot must be occupied");
        // Just confirm the Node case doesn't panic or fail — Node has no
        // routing_entry field so there's no back-edge to verify.
        assert_eq!(stored.read().unwrap().target_index, 42);
    }

    // ---------- §7.2 relay-inclusion ----------

    #[test]
    fn fresher_seq_replaces_stored_entry() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            10,
            50,
            false,
        );

        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 20, 10, 1, false),
            )
            .unwrap();

        let stored = state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap();
        let e = stored.read().unwrap();
        assert_eq!(e.seq_num, SeqNum::from(20u16));
        assert_eq!(e.metric, 20); // 10 + hop_cost(Lan, None) = 10 + 10
    }

    /// Reboot: a huge forward gap under wrap arithmetic must still be
    /// accepted per spec §6.3 / §7.2.
    #[test]
    fn reboot_gap_accepts_new_entry() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            200,
            50,
            false,
        );

        // Incoming seq=30, stored=200: forward gap under wrap is 65_366, > 100 → Reboot.
        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 30, 10, 1, false),
            )
            .unwrap();

        let stored = state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap();
        assert_eq!(stored.read().unwrap().seq_num, SeqNum::from(30u16));
    }

    /// Per §6.3, any gap > 100 (including large backward-looking jumps
    /// under wrap arithmetic) is treated as a reboot and accepted. There
    /// is no "older, drop me" bucket — a peer with a lower seq than we
    /// have stored is presumed to have restarted with a fresh random seed.
    /// Pins this behaviour so a future refactor of `acceptance` doesn't
    /// silently drift into "naive greater-than" semantics.
    #[test]
    fn backward_looking_seq_is_treated_as_reboot_and_accepted() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            50,
            30,
            false,
        );

        // seq=40, stored=50: forward gap under wrap = 65_526 → Reboot bucket.
        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 40, 10, 1, false),
            )
            .unwrap();

        let stored = state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap();
        let e = stored.read().unwrap();
        assert_eq!(e.seq_num, SeqNum::from(40u16), "reboot bucket must replace");
        assert_eq!(e.metric, 20, "new metric = 10 + hop_cost(Lan, None)=10");
    }

    #[test]
    fn same_seq_lower_metric_replaces_stored() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            10,
            50,
            false,
        );

        // Same seq, incoming metric 5 + hop_cost(Lan, None)=10 = 15 < stored 50.
        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 10, 5, 1, false),
            )
            .unwrap();

        let stored = state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap();
        assert_eq!(stored.read().unwrap().metric, 15);
    }

    /// §7.2 requires strict `<` on the metric tiebreak — equal metric must
    /// not overwrite the incumbent (flapping protection).
    #[test]
    fn same_seq_equal_metric_drops() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            10,
            20,
            false,
        );

        // Same seq, new metric 10 + 10 = 20 = stored → strict < fails → drop.
        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 10, 10, 5, false),
            )
            .unwrap();

        let stored = state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap();
        let e = stored.read().unwrap();
        assert_eq!(e.metric, 20);
        assert_eq!(e.hop_count, 0, "stored hop_count preserved (drop path)");
    }

    #[test]
    fn same_seq_higher_metric_drops() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            10,
            20,
            false,
        );

        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 10, 30, 1, false),
            )
            .unwrap();

        assert_eq!(
            state
                .routing_table
                .read()
                .unwrap()
                .get(Space::User, 42)
                .unwrap()
                .read()
                .unwrap()
                .metric,
            20,
        );
    }

    // ---------- local_only monotonicity (§7.4) ----------

    /// Sticky at zero: stored=false remains false even when incoming=true.
    #[test]
    fn local_only_sticky_when_stored_is_false() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(
            &state,
            Space::User,
            42,
            TargetRef::User(user),
            10,
            50,
            false,
        );

        // Fresher seq → accepted; local_only should stay false.
        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 20, 10, 1, true),
            )
            .unwrap();

        assert!(
            !state
                .routing_table
                .read()
                .unwrap()
                .get(Space::User, 42)
                .unwrap()
                .read()
                .unwrap()
                .local_only
        );
    }

    /// Transitions to false: stored=true is overridden by incoming=false.
    #[test]
    fn local_only_transitions_to_false_when_incoming_is_false() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(&state, Space::User, 42, TargetRef::User(user), 10, 50, true);

        state
            .apply_user_entry(
                &default_ctx(peer, 2_000),
                wire_user_entry(5, 20, 10, 1, false),
            )
            .unwrap();

        assert!(
            !state
                .routing_table
                .read()
                .unwrap()
                .get(Space::User, 42)
                .unwrap()
                .read()
                .unwrap()
                .local_only
        );
    }

    #[test]
    fn local_only_empty_slot_uses_incoming_value() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, _) = setup_user_target(&state, 5, 42, target_id);

        state
            .apply_user_entry(
                &default_ctx(peer, 1_000),
                wire_user_entry(5, 1, 10, 1, true),
            )
            .unwrap();

        assert!(
            state
                .routing_table
                .read()
                .unwrap()
                .get(Space::User, 42)
                .unwrap()
                .read()
                .unwrap()
                .local_only
        );
    }
}

// ---------- handle_routing_update ----------

mod handle_routing_update {
    use super::*;
    use crate::router_v2::{
        codec::messages::{Mapping, NodeEntry, RoutingUpdate, UserEntry},
        index::Space,
        seq::SeqNum,
        test_utils::*,
    };
    use libp2p::PeerId;

    const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
    const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;

    /// Adds a neighbour and binds its node_id in node_dict so that any
    /// entry processed downstream can resolve `next_hop`.
    fn setup_neighbour(state: &RouterV2State) -> PeerId {
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        bind_own_dict(
            state,
            Space::Node,
            NEIGHBOUR_IDX_IN_NODE_DICT,
            NEIGHBOUR_NODE_ID,
        );
        peer
    }

    fn empty_update() -> RoutingUpdate {
        RoutingUpdate {
            user_mappings: Vec::new(),
            node_mappings: Vec::new(),
            user_entries: Vec::new(),
            node_entries: Vec::new(),
        }
    }

    #[test]
    fn empty_message_is_noop() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);

        state
            .handle_routing_update(peer, ConnectionModule::Lan, None, empty_update(), 1_000)
            .unwrap();

        assert_eq!(state.users.read().unwrap().len(), 0);
        assert_eq!(state.nodes.read().unwrap().len(), 0);
        assert!(state
            .routing_table
            .read()
            .unwrap()
            .user_entries
            .iter()
            .all(|s| s.is_none()));
    }

    /// The critical §8.7 ordering guarantee: a mapping and an entry for
    /// the same target arriving in one message must both take effect.
    /// This only works if the mapping section is processed before the
    /// entry section (otherwise the entry would fail target lookup).
    #[test]
    fn mapping_then_entry_lands_full_route() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);

        let target_id = [1; 8];
        let msg = RoutingUpdate {
            user_mappings: vec![Mapping {
                abs_idx: 5,
                target_id,
                version: 3,
            }],
            node_mappings: Vec::new(),
            user_entries: vec![UserEntry {
                abs_idx: 5,
                seq: 7,
                metric: 10,
                hop_count: 2,
                local_only: false,
            }],
            node_entries: Vec::new(),
        };

        state
            .handle_routing_update(peer, ConnectionModule::Lan, None, msg, 5_000)
            .unwrap();

        // Mirror binding from the mapping section.
        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(target_id),);
        drop(mirrors);

        // User stub created by the mapping section, with the carried version.
        let users = state.users.read().unwrap();
        let user_arc = users.get(&target_id).expect("stub must exist");
        assert_eq!(user_arc.read().unwrap().profile_version, 3);
        drop(users);

        // Own idx allocated by translate_incoming (in the entry pass).
        let own_idx = state
            .user_dict
            .read()
            .unwrap()
            .idx_of(&target_id)
            .expect("target must be bound in own dict");

        // Routing entry stored at the allocated own_idx.
        let rt = state.routing_table.read().unwrap();
        let stored = rt.get(Space::User, own_idx).expect("entry must be stored");
        let e = stored.read().unwrap();
        assert_eq!(e.seq_num, SeqNum::from(7u16));
        assert_eq!(e.metric, 20); // 10 + hop_cost(Lan, None) = 20
        assert_eq!(e.hop_count, 3);
    }

    #[test]
    fn both_spaces_processed_independently() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);

        let user_id = [1; 8];
        let node_id = [2; 8];
        let msg = RoutingUpdate {
            user_mappings: vec![Mapping {
                abs_idx: 5,
                target_id: user_id,
                version: 1,
            }],
            node_mappings: vec![Mapping {
                abs_idx: 6,
                target_id: node_id,
                version: 2,
            }],
            user_entries: vec![UserEntry {
                abs_idx: 5,
                seq: 1,
                metric: 10,
                hop_count: 1,
                local_only: false,
            }],
            node_entries: vec![NodeEntry {
                abs_idx: 6,
                seq: 1,
                metric: 15,
                hop_count: 1,
                local_only: false,
                manifest_version: 0,
            }],
        };

        state
            .handle_routing_update(peer, ConnectionModule::Lan, None, msg, 1_000)
            .unwrap();

        assert!(state.users.read().unwrap().get(&user_id).is_some());
        assert!(state.nodes.read().unwrap().get(&node_id).is_some());

        let user_idx = state.user_dict.read().unwrap().idx_of(&user_id).unwrap();
        let node_own_idx = state.node_dict.read().unwrap().idx_of(&node_id).unwrap();
        let rt = state.routing_table.read().unwrap();
        assert!(rt.get(Space::User, user_idx).is_some());
        assert!(rt.get(Space::Node, node_own_idx).is_some());
    }

    /// Unknown neighbour (mirrors doesn't have this peer). Each row's
    /// apply_ call handles this internally with Ok; the orchestrator
    /// finishes without side effects.
    #[test]
    fn unknown_neighbour_processes_without_side_effects() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer(); // never added to mirrors

        let msg = RoutingUpdate {
            user_mappings: vec![Mapping {
                abs_idx: 5,
                target_id: [1; 8],
                version: 1,
            }],
            node_mappings: Vec::new(),
            user_entries: Vec::new(),
            node_entries: Vec::new(),
        };

        state
            .handle_routing_update(peer, ConnectionModule::Lan, None, msg, 1_000)
            .unwrap();

        assert_eq!(state.users.read().unwrap().len(), 0);
        assert!(state.mirrors.read().unwrap().get(&peer).is_none());
    }
}

// ---------- handle_index_dump ----------

mod handle_index_dump {
    use super::*;
    use crate::router_v2::{
        codec::messages::{IndexDump, Mapping},
        index::Space,
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
    };

    fn mapping(abs_idx: u16, target_id: [u8; 8], version: u32) -> Mapping {
        Mapping {
            abs_idx,
            target_id,
            version,
        }
    }

    /// Both sections land: mirrors bound in each index space, and stubs
    /// created in `users` / `nodes` carrying the advertised versions.
    #[test]
    fn both_sections_populate_mirrors_and_stubs() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        let dump = IndexDump {
            user_mappings: vec![mapping(5, [1; 8], 42)],
            node_mappings: vec![mapping(9, [2; 8], 99)],
        };

        state.handle_index_dump(peer, dump).unwrap();

        {
            let mirrors = state.mirrors.read().unwrap();
            let nm = mirrors.get(&peer).unwrap();
            assert_eq!(nm.users.id_of(5), Some([1; 8]));
            assert_eq!(nm.nodes.id_of(9), Some([2; 8]));
        }

        let users = state.users.read().unwrap();
        let user_arc = users.get(&[1; 8]).unwrap();
        let u = user_arc.read().unwrap();
        assert_eq!(u.profile_version, 42);
        assert!(u.public_key.is_none(), "stub must not fabricate a key");

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&[2; 8]).unwrap();
        let n = node_arc.read().unwrap();
        // Pull model (§10.8): a dump's node version is an *advertisement*,
        // not a committed manifest_version.
        assert_eq!(n.manifest_version, 0, "nothing committed yet");
        assert_eq!(n.advertised_version, 99);
    }

    /// Regression test for the accumulate-don't-clear decision. §8.4 lets a
    /// sender split an oversized dictionary across several INDEX_DUMPs, and
    /// the message carries no chunk framing, so the receiver cannot tell a
    /// complete dump from chunk 1 of N. Clearing the mirror per §3.6's
    /// literal "SHALL replace" would discard the earlier chunk; this test
    /// fails the moment someone makes that change.
    #[test]
    fn second_dump_accumulates_and_does_not_clear_first() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        state
            .handle_index_dump(
                peer,
                IndexDump {
                    user_mappings: vec![mapping(5, [1; 8], 1)],
                    node_mappings: vec![mapping(5, [11; 8], 1)],
                },
            )
            .unwrap();

        state
            .handle_index_dump(
                peer,
                IndexDump {
                    user_mappings: vec![mapping(9, [2; 8], 1)],
                    node_mappings: vec![mapping(9, [22; 8], 1)],
                },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        let nm = mirrors.get(&peer).unwrap();
        assert_eq!(nm.users.id_of(5), Some([1; 8]), "chunk 1 user survived");
        assert_eq!(nm.users.id_of(9), Some([2; 8]), "chunk 2 user landed");
        assert_eq!(nm.nodes.id_of(5), Some([11; 8]), "chunk 1 node survived");
        assert_eq!(nm.nodes.id_of(9), Some([22; 8]), "chunk 2 node landed");
    }

    /// A dump that rebinds a still-live index delegates to `apply_mapping`'s
    /// teardown path: old routing entry cleared, own index released to
    /// cooldown, own dict unbound, mirror repointed.
    #[test]
    fn dump_rebinding_live_index_clears_old_routing_state() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        let old_id = [10; 8];
        let new_id = [20; 8];
        let own_idx: u16 = 7;

        bind_mirror(&state, peer, Space::User, 5, old_id);
        let old_user = install_user(&state, old_id, 1);
        bind_own_dict(&state, Space::User, own_idx, old_id);

        let entry = Arc::new(RwLock::new(RoutingEntry {
            target_index: own_idx,
            target: TargetRef::User(old_user.clone()),
            seq_num: SeqNum::from(0u16),
            metric: 5,
            next_hop: 0,
            transport: ConnectionModule::Lan,
            last_update: 0,
            hop_count: 1,
            local_only: false,
        }));
        let entry_weak = Arc::downgrade(&entry);
        state
            .routing_table
            .write()
            .unwrap()
            .set(Space::User, own_idx, entry);
        old_user.write().unwrap().routing_entry = Some(entry_weak.clone());

        state
            .handle_index_dump(
                peer,
                IndexDump {
                    user_mappings: vec![mapping(5, new_id, 1)],
                    node_mappings: Vec::new(),
                },
            )
            .unwrap();

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, own_idx)
            .is_none());
        assert!(entry_weak.upgrade().is_none(), "old entry Arc must drop");
        assert_eq!(state.user_dict.read().unwrap().idx_of(&old_id), None);
        assert!(state
            .users_allocator
            .read()
            .unwrap()
            .idx_in_cooldown(own_idx));

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer).unwrap().users.id_of(5), Some(new_id));
    }

    /// The user section is processed before the node section, because node
    /// delegations may reference users the user section introduces.
    #[test]
    fn user_section_is_processed_before_node_section() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        // Same abs_idx in both spaces bound to different ids: if the two
        // sections were applied to the same dictionary, one would clobber
        // the other. The spaces are independent (§3.5).
        state
            .handle_index_dump(
                peer,
                IndexDump {
                    user_mappings: vec![mapping(3, [7; 8], 5)],
                    node_mappings: vec![mapping(3, [8; 8], 6)],
                },
            )
            .unwrap();

        let mirrors = state.mirrors.read().unwrap();
        let nm = mirrors.get(&peer).unwrap();
        assert_eq!(nm.users.id_of(3), Some([7; 8]));
        assert_eq!(nm.nodes.id_of(3), Some([8; 8]));
        drop(mirrors);

        assert_eq!(state.users.read().unwrap().len(), 1);
        assert_eq!(state.nodes.read().unwrap().len(), 1);
    }

    /// A dump from a peer with no mirror (never registered, or already
    /// disconnected) is a silent no-op — `apply_mapping` bails per mapping.
    #[test]
    fn unknown_neighbour_is_noop() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer(); // never added to mirrors

        let dump = IndexDump {
            user_mappings: vec![mapping(5, [1; 8], 42)],
            node_mappings: vec![mapping(9, [2; 8], 99)],
        };

        assert!(state.handle_index_dump(peer, dump).is_ok());
        assert!(state.mirrors.read().unwrap().get(&peer).is_none());
        assert_eq!(state.users.read().unwrap().len(), 0);
        assert_eq!(state.nodes.read().unwrap().len(), 0);
    }

    /// An empty dump is legal — a node with an empty dictionary sends one on
    /// connect — and must not panic or create phantom state.
    #[test]
    fn empty_dump_is_harmless() {
        let (state, _rx) = fresh_state();
        let peer = add_neighbour(&state);

        state
            .handle_index_dump(
                peer,
                IndexDump {
                    user_mappings: Vec::new(),
                    node_mappings: Vec::new(),
                },
            )
            .unwrap();

        assert_eq!(state.users.read().unwrap().len(), 0);
        assert_eq!(state.nodes.read().unwrap().len(), 0);
        assert!(
            state.mirrors.read().unwrap().get(&peer).is_some(),
            "mirror still registered"
        );
    }
}

// ---------- received ----------

mod received {
    use super::*;
    use crate::router_v2::{
        codec::{
            messages::{IndexDump, Mapping, RoutingUpdate, UserEntry},
            Header, RoutingMessage, PROTOCOL_VERSION,
        },
        index::Space,
        test_utils::*,
    };
    use libp2p::PeerId;

    const NEIGHBOUR_NODE_ID: [u8; 8] = [77; 8];
    const NEIGHBOUR_IDX_IN_NODE_DICT: u16 = 500;

    fn setup_neighbour(state: &RouterV2State) -> PeerId {
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, NEIGHBOUR_NODE_ID, ConnectionModule::Lan);
        bind_own_dict(
            state,
            Space::Node,
            NEIGHBOUR_IDX_IN_NODE_DICT,
            NEIGHBOUR_NODE_ID,
        );
        peer
    }

    /// Encode a message with the given type + body bytes into a full wire
    /// frame (4-byte header + body).
    fn frame(msg_type: RoutingMessage, body: &[u8]) -> Vec<u8> {
        let header = Header {
            version: PROTOCOL_VERSION,
            message_type: msg_type,
            payload_len: body.len() as u16,
        };
        let mut out = Vec::new();
        header.encode(&mut out);
        out.extend_from_slice(body);
        out
    }

    /// Encode a full ROUTING_UPDATE message ready for `received()`.
    fn frame_routing_update(msg: &RoutingUpdate) -> Vec<u8> {
        let mut body = Vec::new();
        msg.encode(&mut body).unwrap();
        frame(RoutingMessage::RoutingUpdate, &body)
    }

    fn small_valid_update(target_id: [u8; 8]) -> RoutingUpdate {
        RoutingUpdate {
            user_mappings: vec![Mapping {
                abs_idx: 5,
                target_id,
                version: 1,
            }],
            node_mappings: Vec::new(),
            user_entries: vec![UserEntry {
                abs_idx: 5,
                seq: 1,
                metric: 10,
                hop_count: 1,
                local_only: false,
            }],
            node_entries: Vec::new(),
        }
    }

    #[test]
    fn empty_buf_is_noop() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);

        state
            .received(peer, ConnectionModule::Lan, None, &[], 1_000)
            .unwrap();

        assert_eq!(state.users.read().unwrap().len(), 0);
    }

    #[test]
    fn valid_routing_update_dispatches_to_orchestrator() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let target_id = [1; 8];

        let msg = small_valid_update(target_id);
        let bytes = frame_routing_update(&msg);

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        // Mirror + stub + routing entry all landed via the orchestrator.
        assert!(state.users.read().unwrap().get(&target_id).is_some());
        let own_idx = state.user_dict.read().unwrap().idx_of(&target_id).unwrap();
        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, own_idx)
            .is_some());
    }

    /// Two messages back-to-back must both be processed. This pins the
    /// frame-advancement math (advance `buf` by `4 + payload_len`, not
    /// just `payload_len`) — the bug that would silently corrupt the
    /// next header.
    #[test]
    fn multiple_valid_messages_in_batch_are_all_processed() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let target_a = [1; 8];
        let target_b = [2; 8];

        let mut bytes = frame_routing_update(&small_valid_update(target_a));
        bytes.extend(frame_routing_update(&small_valid_update(target_b)));

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        let users = state.users.read().unwrap();
        assert!(users.get(&target_a).is_some(), "first message applied");
        assert!(users.get(&target_b).is_some(), "second message applied");
    }

    /// Forward-compat behaviour (§8.2): a message with an unknown version
    /// must be skipped past (using payload_len) so that a subsequent
    /// valid message is still processed.
    #[test]
    fn bad_version_skips_and_processes_subsequent_valid_message() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let target_id = [3; 8];

        // Fake message with unknown version 0xFE and payload_len 8.
        let bad_body = [0xAAu8; 8];
        let mut bytes = vec![0xFE, 0x01, 0x00, 0x08];
        bytes.extend_from_slice(&bad_body);

        // Then a valid RoutingUpdate.
        bytes.extend(frame_routing_update(&small_valid_update(target_id)));

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        assert!(
            state.users.read().unwrap().get(&target_id).is_some(),
            "valid message following a BadVersion must still be processed",
        );
    }

    /// Header says payload_len=100, but only 4 bytes of body follow.
    /// The receive loop should log-and-return without applying anything.
    #[test]
    fn truncated_body_returns_without_partial_state() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);

        let mut bytes = Vec::new();
        // Header: version=1, type=RoutingUpdate=1, payload_len=100.
        bytes.extend_from_slice(&[PROTOCOL_VERSION, 0x01, 0x00, 0x64]);
        // Only 4 bytes of body, not 100.
        bytes.extend_from_slice(&[0x00; 4]);

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        assert_eq!(state.users.read().unwrap().len(), 0);
    }

    /// A body that fails to decode must not desync the frame loop: alignment
    /// comes from the header's `payload_len`, which is consumed before the body
    /// is parsed, so the next message still processes.
    #[test]
    fn undecodable_body_is_skipped_and_next_processed() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let target_id = [4; 8];

        // A ManifestDelta too short to be a valid body, then a good update.
        let delta_body = [0x00u8; 2];
        let mut bytes = frame(RoutingMessage::ManifestDelta, &delta_body);
        bytes.extend(frame_routing_update(&small_valid_update(target_id)));

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        // The malformed delta was dropped, then the RoutingUpdate applied.
        assert!(state.users.read().unwrap().get(&target_id).is_some());
    }

    /// INDEX_DUMP now has a handler, so it must actually populate mirrors
    /// rather than falling into the catch-all — and the message after it
    /// must still be processed.
    #[test]
    fn index_dump_is_dispatched_to_handler() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let dumped_id = [9; 8];
        let target_id = [4; 8];

        let dump = IndexDump {
            user_mappings: vec![Mapping {
                abs_idx: 12,
                target_id: dumped_id,
                version: 77,
            }],
            node_mappings: Vec::new(),
        };
        let mut body = Vec::new();
        dump.encode(&mut body).unwrap();

        let mut bytes = frame(RoutingMessage::IndexDump, &body);
        bytes.extend(frame_routing_update(&small_valid_update(target_id)));

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        // The dump was handled, not skipped.
        {
            let mirrors = state.mirrors.read().unwrap();
            assert_eq!(
                mirrors.get(&peer).unwrap().users.id_of(12),
                Some(dumped_id),
                "INDEX_DUMP must reach handle_index_dump"
            );
        }
        let users = state.users.read().unwrap();
        let dumped_arc = users.get(&dumped_id).unwrap();
        assert_eq!(dumped_arc.read().unwrap().profile_version, 77);

        // Frame alignment preserved: the following update still applied.
        assert!(users.get(&target_id).is_some());
    }

    #[test]
    fn malformed_routing_update_body_does_not_corrupt_frame_alignment() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let target_id = [5; 8];

        // Header claims payload_len=4, but 4 bytes of garbage isn't a
        // valid RoutingUpdate body — decoder fails. Frame alignment is
        // preserved because buf was advanced before the decode attempt,
        // so the next message still processes.
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&[PROTOCOL_VERSION, 0x01, 0x00, 0x04]);
        bytes.extend_from_slice(&[0xFF; 4]); // garbage body
        bytes.extend(frame_routing_update(&small_valid_update(target_id)));

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        assert!(
            state.users.read().unwrap().get(&target_id).is_some(),
            "valid message after a body-decode failure must still be processed",
        );
    }
}

// ---------- hosted user / neighbour node registration (§3.2, §3.5) ----------

mod self_and_neighbour_registration {
    use super::*;
    use crate::router_v2::{index::RESERVED_INDEX, index::Space, test_utils::*, PropagationForm};

    /// Drains the pending introduction marks for `space`.
    fn take_marks(state: &RouterV2State, space: Space) -> std::collections::HashSet<u16> {
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(space)
    }

    // ----- register_hosted_user -----

    /// All three effects must land together. `pending_introductions` resolves a
    /// mark through both the dictionary and the users map and silently discards
    /// it if either is missing, so a partial write binds the index but never
    /// introduces it — leaving peers unable to translate index 0.
    #[test]
    fn register_hosted_user_binds_record_and_marks() {
        let (state, _rx) = fresh_state();
        let user_id = [42; 8];

        state.register_hosted_user(user_id, 7);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some(user_id),
            "hosted user must occupy RESERVED_INDEX in the user space"
        );

        let users = state.users.read().unwrap();
        let user_arc = users.get(&user_id).expect("User record created");
        assert_eq!(user_arc.read().unwrap().profile_version, 7);
        drop(users);

        assert!(
            take_marks(&state, Space::User).contains(&RESERVED_INDEX),
            "the binding must be queued for introduction"
        );
    }

    /// The whole point of the binding: `pending_introductions` must resolve it
    /// into a real mapping. This is the assertion that would have caught the
    /// original bug, where index 0 was never introduced and every peer dropped
    /// the origin's user entry.
    #[test]
    fn register_hosted_user_produces_a_resolvable_introduction() {
        let (state, _rx) = fresh_state();
        let user_id = [42; 8];

        state.register_hosted_user(user_id, 7);

        let intros = state.pending_introductions(Space::User);
        assert_eq!(
            intros,
            vec![(RESERVED_INDEX, user_id, 7)],
            "introduction must carry index, id and profile_version"
        );
    }

    /// Called on every startup, so repeats must not churn state.
    #[test]
    fn register_hosted_user_is_idempotent() {
        let (state, _rx) = fresh_state();
        let user_id = [42; 8];

        state.register_hosted_user(user_id, 1);
        state.register_hosted_user(user_id, 1);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some(user_id)
        );
        assert_eq!(state.users.read().unwrap().len(), 1, "no duplicate record");
    }

    /// A later call with a fresher profile_version updates the existing record
    /// rather than being ignored or inserting a second one.
    #[test]
    fn register_hosted_user_updates_profile_version_on_existing_record() {
        let (state, _rx) = fresh_state();
        let user_id = [42; 8];

        state.register_hosted_user(user_id, 1);
        state.register_hosted_user(user_id, 9);

        let users = state.users.read().unwrap();
        let user_arc = users.get(&user_id).unwrap();
        assert_eq!(user_arc.read().unwrap().profile_version, 9);
        assert_eq!(users.len(), 1);
    }

    /// §3.2 reading (a): a second hosted user puts the node in *node* form,
    /// where users are named by the manifest rather than by routing entries —
    /// so the newcomer gets no user index at all, and the first keeps the
    /// reserved slot until the form transition releases it.
    ///
    /// The eviction this guards against is real: `IndexDictionary::bind`
    /// replaces whatever occupies an index, so binding every hosted user at
    /// RESERVED_INDEX would silently repoint peers' index 0 at a different user.
    #[test]
    fn second_hosted_user_does_not_evict_the_first() {
        let (state, _rx) = fresh_state();
        let first = [1; 8];
        let second = [2; 8];

        state.register_hosted_user(first, 1);
        state.register_hosted_user(second, 1);

        let dict = state.user_dict.read().unwrap();
        assert_eq!(
            dict.id_of(RESERVED_INDEX),
            Some(first),
            "the first hosted user keeps the reserved slot"
        );
        assert!(
            dict.idx_of(&second).is_none(),
            "a node-form host assigns its users no user index"
        );
        drop(dict);

        assert_eq!(state.hosted_user_ids().len(), 2);
        assert_eq!(
            state.desired_propagation_form(),
            PropagationForm::Node,
            "two hosted users is the §3.2 node-form trigger"
        );
    }

    /// Only an indexed user can be introduced. The second has no index, so the
    /// user space has exactly one introduction pending — and it is the first
    /// user at the reserved slot.
    #[test]
    fn only_the_indexed_hosted_user_is_introduced() {
        let (state, _rx) = fresh_state();
        let first = [1; 8];
        let second = [2; 8];

        state.register_hosted_user(first, 3);
        state.register_hosted_user(second, 4);

        assert_eq!(
            state.pending_introductions(Space::User),
            vec![(RESERVED_INDEX, first, 3)]
        );
    }

    /// Re-registering must not disturb the reserved slot or duplicate records,
    /// whichever form the node is in.
    #[test]
    fn repeat_registration_is_stable_across_the_form_boundary() {
        let (state, _rx) = fresh_state();
        let first = [1; 8];
        let second = [2; 8];

        state.register_hosted_user(first, 1);
        state.register_hosted_user(second, 1);

        state.register_hosted_user(first, 2);
        state.register_hosted_user(second, 2);

        let dict = state.user_dict.read().unwrap();
        assert_eq!(dict.id_of(RESERVED_INDEX), Some(first));
        assert!(dict.idx_of(&second).is_none());
        drop(dict);
        assert_eq!(state.users.read().unwrap().len(), 2);
        assert_eq!(state.hosted_user_ids().len(), 2);
    }

    // ----- register_neighbour_node -----

    /// A neighbour must get an own-side node index: a routing entry's next_hop
    /// is a node index, and `translate_incoming` only allocates for targets
    /// named by incoming *entries*. Without this a neighbour that is only ever
    /// a next hop is never allocated one and every entry it sends is rejected.
    #[test]
    fn register_neighbour_node_allocates_index_record_and_mark() {
        let (state, _rx) = fresh_state();
        let node_id = [77; 8];
        let key = fresh_multikey();

        state.register_neighbour_node(node_id, Some(key));

        let idx = state
            .node_dict
            .read()
            .unwrap()
            .idx_of(&node_id)
            .expect("neighbour allocated a node index");
        assert_ne!(
            idx, RESERVED_INDEX,
            "the allocator must never hand out the reserved self index"
        );

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&node_id).expect("Node record created");
        assert!(
            node_arc.read().unwrap().public_key.is_some(),
            "the key from try_from_peer_id must be retained for §8.8 verification"
        );
        drop(nodes);

        assert!(
            take_marks(&state, Space::Node).contains(&idx),
            "binding must be queued for introduction"
        );
    }

    /// `ping_event` gates on `add_neighbour_transport`, but registration must be
    /// safe under repeats regardless — and must not consume a fresh index each
    /// time, which would exhaust the allocator on a flapping link.
    #[test]
    fn register_neighbour_node_is_idempotent() {
        let (state, _rx) = fresh_state();
        let node_id = [77; 8];

        state.register_neighbour_node(node_id, Some(fresh_multikey()));
        let first = state.node_dict.read().unwrap().idx_of(&node_id).unwrap();

        for _ in 0..5 {
            state.register_neighbour_node(node_id, Some(fresh_multikey()));
        }

        assert_eq!(
            state.node_dict.read().unwrap().idx_of(&node_id),
            Some(first),
            "repeat registration must not reallocate"
        );
        assert_eq!(state.nodes.read().unwrap().len(), 1, "no duplicate record");
    }

    /// Stubs built by `apply_mapping` carry `public_key: None`. Registering with
    /// a key must upgrade such a stub in place rather than leaving it keyless —
    /// §8.8 cannot verify a manifest without it.
    #[test]
    fn register_neighbour_node_upgrades_a_keyless_stub() {
        let (state, _rx) = fresh_state();
        let node_id = [77; 8];

        state.register_neighbour_node(node_id, None);
        {
            let nodes = state.nodes.read().unwrap();
            let node_arc = nodes.get(&node_id).unwrap();
            assert!(node_arc.read().unwrap().public_key.is_none());
        }

        state.register_neighbour_node(node_id, Some(fresh_multikey()));

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&node_id).unwrap();
        assert!(
            node_arc.read().unwrap().public_key.is_some(),
            "a later call carrying a key must fill in the stub"
        );
    }

    /// The inverse: a call without a key must never clear one we already hold.
    #[test]
    fn register_neighbour_node_never_downgrades_a_known_key() {
        let (state, _rx) = fresh_state();
        let node_id = [77; 8];

        state.register_neighbour_node(node_id, Some(fresh_multikey()));
        state.register_neighbour_node(node_id, None);

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&node_id).unwrap();
        assert!(
            node_arc.read().unwrap().public_key.is_some(),
            "None must not overwrite a known key"
        );
    }

    /// Distinct neighbours get distinct indexes, and neither collides with the
    /// host's own node binding at RESERVED_INDEX.
    #[test]
    fn distinct_neighbours_get_distinct_indexes() {
        let (state, _rx) = fresh_state();

        state.register_neighbour_node([10; 8], None);
        state.register_neighbour_node([20; 8], None);

        let dict = state.node_dict.read().unwrap();
        let a = dict.idx_of(&[10; 8]).unwrap();
        let b = dict.idx_of(&[20; 8]).unwrap();
        assert_ne!(a, b);
        assert_ne!(a, RESERVED_INDEX);
        assert_ne!(b, RESERVED_INDEX);
    }

    /// §3.5 ties each reserved index to a propagation form. In user form the
    /// hosted user holds user-space 0x0000 and node-space 0x0000 is *unbound* —
    /// the node self-binding only exists while propagating as a node entry.
    #[test]
    fn only_the_active_form_holds_a_reserved_binding() {
        let (state, _rx) = fresh_state();
        let user_id = [42; 8];

        state.register_hosted_user(user_id, 1);
        state.register_neighbour_node([77; 8], None);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some(user_id)
        );
        assert!(
            state.node_dict.read().unwrap().id_of(RESERVED_INDEX).is_none(),
            "node-space reserved index stays unbound while in user form"
        );
    }

    /// Neighbours must never be handed the reserved slot, whichever form the
    /// node is in.
    #[test]
    fn neighbour_registration_never_touches_the_reserved_slot() {
        let (state, _rx) = fresh_state();

        state.register_neighbour_node([77; 8], None);
        state.register_neighbour_node([88; 8], None);

        let dict = state.node_dict.read().unwrap();
        assert!(dict.id_of(RESERVED_INDEX).is_none());
        assert_ne!(dict.idx_of(&[77; 8]).unwrap(), RESERVED_INDEX);
        assert_ne!(dict.idx_of(&[88; 8]).unwrap(), RESERVED_INDEX);
    }
}

// ---------- propagation form (§3.2, §3.5) ----------

mod propagation_form {
    use super::*;
    use crate::router_v2::{index::RESERVED_INDEX, index::Space, test_utils::*, PropagationForm};

    /// One hosted user, LAN-only: the §3.2 default.
    #[test]
    fn single_user_no_internet_is_user_form() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Lan);

        assert_eq!(state.desired_propagation_form(), PropagationForm::User);
    }

    /// Remote users must not count toward the multi-user trigger. Every
    /// neighbour's user gets a stub in `users`, so counting the map instead of
    /// the `is_hosted` flag would flip a plain two-node LAN into node form.
    #[test]
    fn remote_users_do_not_trigger_node_form() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        install_user(&state, [2; 8], 0); // learned from a neighbour
        install_user(&state, [3; 8], 0);

        assert_eq!(state.users.read().unwrap().len(), 3);
        assert_eq!(state.hosted_user_ids().len(), 1);
        assert_eq!(state.desired_propagation_form(), PropagationForm::User);
    }

    /// A user can be seen through a neighbour before the local account loads,
    /// leaving a stub with `is_hosted: false`. Registering it must upgrade the
    /// existing record rather than only setting the flag on a fresh insert.
    #[test]
    fn registering_an_existing_remote_stub_marks_it_hosted() {
        let (state, _rx) = fresh_state();
        install_user(&state, [1; 8], 5); // remote stub

        state.register_hosted_user([1; 8], 6);

        assert_eq!(state.hosted_user_ids(), vec![[1; 8]]);
        assert_eq!(state.users.read().unwrap().len(), 1, "no duplicate record");
    }

    #[test]
    fn second_hosted_user_triggers_node_form() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        assert_eq!(state.desired_propagation_form(), PropagationForm::User);

        state.register_hosted_user([2; 8], 0);
        assert_eq!(state.desired_propagation_form(), PropagationForm::Node);
    }

    /// Spec line 206 keys this on an active INTERNET *connection*, which is
    /// what a neighbour entry represents — not on a bound listener.
    #[test]
    fn internet_neighbour_triggers_node_form() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.add_neighbour_transport(fresh_peer(), [77; 8], ConnectionModule::Internet);

        assert_eq!(state.desired_propagation_form(), PropagationForm::Node);
    }

    /// §3.5 + reading (a): in node form the hosted users are named by the
    /// manifest, not by routing entries, so they hold no user index at all.
    #[test]
    fn switching_to_node_form_releases_hosted_user_indexes() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8])
        );

        state.register_hosted_user([2; 8], 0);
        assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

        let dict = state.user_dict.read().unwrap();
        assert!(
            dict.id_of(RESERVED_INDEX).is_none(),
            "user-space reserved slot is released on entering node form"
        );
        assert!(dict.idx_of(&[1; 8]).is_none());
        assert!(dict.idx_of(&[2; 8]).is_none());
        drop(dict);

        assert!(
            state
                .reintroduction_tracker
                .write()
                .unwrap()
                .take_pending(Space::Node)
                .contains(&RESERVED_INDEX),
            "the node self-binding must be introduced on entering node form"
        );
    }

    /// §3.5: exactly one space holds a self-binding at a time. Entering node
    /// form must bind node-space 0x0000 to the host, and it needs a `Node`
    /// record for itself or `pending_introductions` discards the mark as an
    /// orphan and neighbours never learn the binding.
    #[test]
    fn entering_node_form_binds_the_node_reserved_index() {
        let (state, _rx) = fresh_state();
        let host_node_id = state.host_mk.to_id();
        state.register_hosted_user([1; 8], 0);
        state.register_hosted_user([2; 8], 0);

        assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

        assert_eq!(
            state.node_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some(host_node_id),
            "host takes the node-space reserved slot in node form"
        );
        assert!(
            state.nodes.read().unwrap().get(&host_node_id).is_some(),
            "a Node record for the host must exist for the introduction to resolve"
        );

        let intros = state.pending_introductions(Space::Node);
        assert!(
            intros.iter().any(|(idx, id, _)| *idx == RESERVED_INDEX && *id == host_node_id),
            "the node self-binding must resolve into a real introduction, not an orphan mark"
        );
    }

    /// The inverse: leaving node form releases the node-space self-binding, so
    /// the two reserved slots are never bound simultaneously.
    #[test]
    fn leaving_node_form_releases_the_node_reserved_index() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.register_hosted_user([2; 8], 0);
        assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

        state.unregister_hosted_user([2; 8]);
        assert_eq!(state.sync_propagation_form(0), PropagationForm::User);

        assert!(
            state.node_dict.read().unwrap().id_of(RESERVED_INDEX).is_none(),
            "node-space reserved slot is released on returning to user form"
        );
        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8])
        );
    }

    /// Dropping back to a single hosted user returns the node to user form and
    /// puts that user back at RESERVED_INDEX.
    #[test]
    fn switching_back_to_user_form_rebinds_reserved_index() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.register_hosted_user([2; 8], 0);
        assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

        state.unregister_hosted_user([2; 8]);
        assert_eq!(state.sync_propagation_form(0), PropagationForm::User);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8]),
            "the surviving hosted user reclaims the reserved slot"
        );
    }

    /// Reconciling an unchanged form must be a no-op — it runs every origin
    /// tick, so churning indexes here would reintroduce bindings forever.
    #[test]
    fn sync_is_a_noop_when_the_form_has_not_changed() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        assert_eq!(state.sync_propagation_form(0), PropagationForm::User);

        // drain marks from registration
        let _ = state.pending_introductions(Space::User);

        assert_eq!(state.sync_propagation_form(0), PropagationForm::User);
        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8])
        );
        assert!(
            state.pending_introductions(Space::User).is_empty(),
            "an unchanged form must not queue new introductions"
        );
    }

    /// Releasing an index must also drop its pending introduction. Otherwise
    /// the mark outlives the binding and `pending_introductions` reports an
    /// "orphan mark" — and worse, the index can be reallocated to a different
    /// target while a stale mark still points at it.
    #[test]
    fn releasing_an_index_clears_its_pending_mark() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.register_hosted_user([2; 8], 0);

        // The first registration queued an introduction at the reserved slot;
        // the form switch releases that index, so the mark must go with it.
        assert_eq!(state.sync_propagation_form(0), PropagationForm::Node);

        let pending = state
            .reintroduction_tracker
            .write()
            .unwrap()
            .take_pending(Space::User);
        assert!(
            !pending.contains(&RESERVED_INDEX),
            "reserved slot's mark must be cleared with its binding"
        );
        assert!(pending.is_empty(), "no user-space marks should survive");
    }

    // ----- unregister_hosted_user -----

    #[test]
    fn unregister_releases_index_and_record() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);

        state.unregister_hosted_user([1; 8]);

        assert!(state.user_dict.read().unwrap().id_of(RESERVED_INDEX).is_none());
        assert!(state.users.read().unwrap().get(&[1; 8]).is_none());
        assert_eq!(state.hosted_user_ids().len(), 0);
    }

    /// §3.5 keeps exactly one hosted user at 0x0000. Removing the one that
    /// holds it must promote a survivor, not leave the slot empty.
    #[test]
    fn unregister_promotes_a_survivor_into_the_reserved_slot() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.register_hosted_user([2; 8], 0);
        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8])
        );

        state.unregister_hosted_user([1; 8]);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([2; 8]),
            "the remaining hosted user takes the reserved slot"
        );
    }

    /// Removing an *un-indexed* hosted user — the normal case in node form,
    /// where only the reserved slot is ever bound — must leave that slot alone.
    #[test]
    fn unregister_unindexed_user_leaves_reserved_slot_intact() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);
        state.register_hosted_user([2; 8], 0);
        assert!(state.user_dict.read().unwrap().idx_of(&[2; 8]).is_none());

        state.unregister_hosted_user([2; 8]);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8]),
            "the reserved slot is untouched by removing an un-indexed user"
        );
        assert_eq!(state.hosted_user_ids(), vec![[1; 8]]);
    }

    /// Unregistering something we never hosted must not panic or disturb state.
    #[test]
    fn unregister_unknown_user_is_a_noop() {
        let (state, _rx) = fresh_state();
        state.register_hosted_user([1; 8], 0);

        state.unregister_hosted_user([9; 8]);

        assert_eq!(
            state.user_dict.read().unwrap().id_of(RESERVED_INDEX),
            Some([1; 8])
        );
        assert_eq!(state.hosted_user_ids().len(), 1);
    }
}

// ---------- neighbour transport registration (§4.2) ----------

mod neighbour_transport {
    use super::*;
    use crate::router_v2::test_utils::*;

    /// The first transport for an unknown peer is new: the mirror is created
    /// and the caller is told to run bootstrap work.
    #[test]
    fn first_transport_for_new_peer_reports_new() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();

        assert!(
            state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
            "first registration must report the pair as newly reachable"
        );

        let mirrors = state.mirrors.read().unwrap();
        let info = mirrors.get(&peer).expect("mirror created");
        assert_eq!(info.node_id, [77; 8]);
        assert!(info.transports.contains(&ConnectionModule::Lan));
    }

    /// Re-registering the same transport must report `false`. This is what
    /// stops `on_neighbour_connect` re-sending a full INDEX_DUMP on every
    /// ping, since ping_event fires continuously for a live neighbour.
    #[test]
    fn repeat_registration_of_same_transport_reports_not_new() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();

        assert!(state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan));

        for _ in 0..5 {
            assert!(
                !state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
                "repeat pings must not re-trigger bootstrap"
            );
        }

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(
            mirrors.get(&peer).unwrap().transports.len(),
            1,
            "transport set must not grow on repeats"
        );
    }

    /// A second, distinct transport to an already-known peer is also new —
    /// §4.2 tracks reachability per (peer, transport) pair, and the new
    /// transport needs its own INDEX_DUMP.
    #[test]
    fn second_distinct_transport_reports_new() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();

        assert!(state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan));
        assert!(
            state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet),
            "a distinct transport to a known peer is newly reachable"
        );

        let mirrors = state.mirrors.read().unwrap();
        let info = mirrors.get(&peer).unwrap();
        assert_eq!(info.transports.len(), 2);
        assert!(info.transports.contains(&ConnectionModule::Lan));
        assert!(info.transports.contains(&ConnectionModule::Internet));
    }

    /// Distinct peers are tracked independently.
    #[test]
    fn distinct_peers_are_independent() {
        let (state, _rx) = fresh_state();
        let peer_a = fresh_peer();
        let peer_b = fresh_peer();

        assert!(state.add_neighbour_transport(peer_a, [10; 8], ConnectionModule::Lan));
        assert!(
            state.add_neighbour_transport(peer_b, [20; 8], ConnectionModule::Lan),
            "a different peer on the same transport is still new"
        );

        let mirrors = state.mirrors.read().unwrap();
        assert_eq!(mirrors.get(&peer_a).unwrap().node_id, [10; 8]);
        assert_eq!(mirrors.get(&peer_b).unwrap().node_id, [20; 8]);
    }

    /// Dropping the last transport removes the mirror, so a later reconnect
    /// reports `true` again and bootstrap re-runs. Without this the reconnect
    /// path would silently stop sending INDEX_DUMP.
    #[test]
    fn reconnect_after_full_disconnect_reports_new_again() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();

        assert!(state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan));
        state.remove_neighbour_transport(peer, ConnectionModule::Lan);
        assert!(
            state.mirrors.read().unwrap().get(&peer).is_none(),
            "last transport removed → mirror dropped"
        );

        assert!(
            state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
            "reconnect must be reported as new so bootstrap re-runs"
        );
    }

    /// Dropping one of two transports keeps the mirror alive, and re-adding
    /// only that transport is new — the surviving one is not.
    #[test]
    fn partial_disconnect_keeps_mirror_and_other_transport() {
        let (state, _rx) = fresh_state();
        let peer = fresh_peer();

        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        state.remove_neighbour_transport(peer, ConnectionModule::Lan);

        {
            let mirrors = state.mirrors.read().unwrap();
            let info = mirrors.get(&peer).expect("mirror survives partial drop");
            assert_eq!(info.transports.len(), 1);
            assert!(info.transports.contains(&ConnectionModule::Internet));
        }

        assert!(
            state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan),
            "the dropped transport is new again"
        );
        assert!(
            !state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet),
            "the surviving transport is not new"
        );
    }
}

// ---------- propagation (Phase 8 checkpoint A) ----------

mod propagation {
    use super::*;
    use crate::router_v2::{
        codec::{messages::RoutingUpdate, Header, RoutingMessage},
        index::Space,
        propagation::{
            blocked_by_split_horizon, compute_outgoing_local_only, should_propagate, tick_origin,
        },
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
        Sphere,
    };

    /// Build a RoutingEntry with the fields the caller cares about.
    /// Other fields get harmless defaults.
    fn make_entry(
        target: TargetRef,
        next_hop: u16,
        transport: ConnectionModule,
        local_only: bool,
    ) -> RoutingEntry {
        RoutingEntry {
            target_index: 0,
            target,
            seq_num: SeqNum::from(0u16),
            metric: 0,
            next_hop,
            transport,
            last_update: 0,
            hop_count: 0,
            local_only,
        }
    }

    /// Decode a framed OutboundMsg body back into a RoutingUpdate for
    /// tick_origin/tick_relay tests to inspect the wire output.
    fn decode_frame(bytes: &[u8]) -> RoutingUpdate {
        let (header, body_slice) = Header::decode(bytes).expect("frame header");
        assert_eq!(header.message_type, RoutingMessage::RoutingUpdate);
        let payload = &body_slice[..header.payload_len as usize];
        RoutingUpdate::decode(payload).expect("routing-update body")
    }

    // ---------- blocked_by_split_horizon ----------

    #[test]
    fn split_horizon_blocks_when_next_hop_is_outgoing_neighbour() {
        let (state, _rx) = fresh_state();
        let target = install_user(&state, [1; 8], 0);
        let neighbour_id = [42; 8];
        bind_own_dict(&state, Space::Node, 7, neighbour_id);

        let entry = make_entry(TargetRef::User(target), 7, ConnectionModule::Lan, false);
        assert!(blocked_by_split_horizon(&state, &entry, neighbour_id));
    }

    #[test]
    fn split_horizon_allows_when_next_hop_is_different_neighbour() {
        let (state, _rx) = fresh_state();
        let target = install_user(&state, [1; 8], 0);
        bind_own_dict(&state, Space::Node, 7, [42; 8]);

        let entry = make_entry(TargetRef::User(target), 7, ConnectionModule::Lan, false);
        assert!(!blocked_by_split_horizon(&state, &entry, [99; 8]));
    }

    /// Defensive: an entry pointing at an unresolvable next_hop is blocked
    /// rather than sprayed onto every neighbour.
    #[test]
    fn split_horizon_blocks_when_next_hop_unresolvable() {
        let (state, _rx) = fresh_state();
        let target = install_user(&state, [1; 8], 0);

        let entry = make_entry(TargetRef::User(target), 99, ConnectionModule::Lan, false);
        assert!(blocked_by_split_horizon(&state, &entry, [42; 8]));
    }

    // ---------- should_propagate ----------

    #[test]
    fn should_propagate_local_outgoing_allows_local_learned() {
        let (state, _rx) = fresh_state();
        let target = install_user(&state, [1; 8], 0);
        let entry = make_entry(TargetRef::User(target), 0, ConnectionModule::Lan, false);
        assert!(should_propagate(&entry, Sphere::Local));
    }

    #[test]
    fn should_propagate_local_outgoing_blocks_internet_learned() {
        let (state, _rx) = fresh_state();
        let target = install_user(&state, [1; 8], 0);
        let entry = make_entry(
            TargetRef::User(target),
            0,
            ConnectionModule::Internet,
            false,
        );
        assert!(!should_propagate(&entry, Sphere::Local));
    }

    #[test]
    fn should_propagate_internet_outgoing_allows_gateway_node() {
        let (state, _rx) = fresh_state();
        let target = install_node(&state, [1; 8], 0, true);
        let entry = make_entry(TargetRef::Node(target), 0, ConnectionModule::Lan, false);
        assert!(should_propagate(&entry, Sphere::Internet));
    }

    #[test]
    fn should_propagate_internet_outgoing_blocks_non_gateway_node() {
        let (state, _rx) = fresh_state();
        let target = install_node(&state, [1; 8], 0, false);
        let entry = make_entry(TargetRef::Node(target), 0, ConnectionModule::Lan, false);
        assert!(!should_propagate(&entry, Sphere::Internet));
    }

    /// User targets never cross the membrane upward, regardless of where
    /// they were learned.
    #[test]
    fn should_propagate_internet_outgoing_blocks_user_targets() {
        let (state, _rx) = fresh_state();
        let target = install_user(&state, [1; 8], 0);
        let entry = make_entry(
            TargetRef::User(target),
            0,
            ConnectionModule::Internet,
            false,
        );
        assert!(!should_propagate(&entry, Sphere::Internet));
    }

    // ---------- compute_outgoing_local_only ----------

    #[test]
    fn outgoing_local_only_internet_always_false() {
        assert!(!compute_outgoing_local_only(false, Sphere::Internet));
        assert!(!compute_outgoing_local_only(true, Sphere::Internet));
    }

    #[test]
    fn outgoing_local_only_local_passes_stored_through() {
        assert!(!compute_outgoing_local_only(false, Sphere::Local));
        assert!(compute_outgoing_local_only(true, Sphere::Local));
    }

    // ---------- tick_origin ----------

    #[test]
    fn tick_origin_with_no_neighbours_pushes_nothing() {
        let (state, mut rx) = fresh_state();

        let before = state.seq_num.read().unwrap().value();
        tick_origin(&state, 0);
        let after = state.seq_num.read().unwrap().value();

        // seq_num always increments once per tick, even with no neighbours.
        assert_eq!(after, before.wrapping_add(1));
        assert!(rx.try_recv().is_err(), "no neighbours → no messages");
    }

    /// One Lan neighbour → one message pushed with local_only=1 (§7.4
    /// origin rule for Local-outgoing).
    #[test]
    fn tick_origin_one_lan_neighbour_pushes_one_message_with_local_only_true() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

        tick_origin(&state, 0);

        let msg = rx.try_recv().expect("one outbound expected");
        assert_eq!(msg.peer, peer);
        assert_eq!(msg.transport, ConnectionModule::Lan);
        assert!(rx.try_recv().is_err(), "no more outbounds");

        let update = decode_frame(&msg.bytes);
        assert_eq!(update.user_entries.len(), 1);
        assert!(update.node_entries.is_empty());
        let entry = &update.user_entries[0];
        assert_eq!(entry.abs_idx, 0, "origin uses RESERVED_INDEX");
        assert_eq!(entry.metric, 0);
        assert_eq!(entry.hop_count, 0);
        assert!(entry.local_only, "Local-outgoing → wire local_only=1");
    }

    /// One Internet neighbour → one message with local_only=0 (§7.4
    /// origin rule for Internet-outgoing).
    ///
    /// An INTERNET neighbour is also §3.2's gateway trigger, so the node is in
    /// *node* form here and originates a node entry rather than a user entry.
    /// Setting up a neighbour transport in a test therefore decides which entry
    /// section gets populated — `desired_propagation_form` reads `mirrors`.
    #[test]
    fn tick_origin_one_internet_neighbour_pushes_message_with_local_only_false() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        tick_origin(&state, 0);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert!(
            update.user_entries.is_empty(),
            "an INTERNET neighbour puts the node in node form (§3.2)"
        );
        assert_eq!(update.node_entries.len(), 1);
        assert!(!update.node_entries[0].local_only);
    }

    /// A neighbour reachable on two transports gets *two* outbound
    /// messages this tick — one per (peer, transport) pair (§4.2).
    #[test]
    fn tick_origin_multi_transport_neighbour_pushes_one_per_transport() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        tick_origin(&state, 0);

        let mut got_transports = Vec::new();
        while let Ok(msg) = rx.try_recv() {
            assert_eq!(msg.peer, peer);
            got_transports.push(msg.transport);
        }
        got_transports.sort_by_key(|t| format!("{t:?}"));
        assert_eq!(got_transports.len(), 2);
        assert!(got_transports.contains(&ConnectionModule::Lan));
        assert!(got_transports.contains(&ConnectionModule::Internet));
    }

    /// Pending introductions must be attached to every neighbour's message
    /// in the mapping section corresponding to the origin space.
    #[test]
    fn tick_origin_attaches_pending_introductions_to_mapping_section() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

        // Set up an introduction: install a user, bind dict, mark tracker.
        let user_id = [11; 8];
        install_user(&state, user_id, 3);
        state.user_dict.write().unwrap().bind(5, user_id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 5);

        tick_origin(&state, 0);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert_eq!(update.user_mappings.len(), 1);
        assert!(update.node_mappings.is_empty());
        assert_eq!(update.user_mappings[0].abs_idx, 5);
        assert_eq!(update.user_mappings[0].target_id, user_id);
        assert_eq!(update.user_mappings[0].version, 3);
    }

    /// Two neighbours + one introduction → the *same* mapping section
    /// appears in *both* outbound messages. Drain the tracker only once,
    /// but attach to all neighbours (§3.8).
    #[test]
    fn tick_origin_same_intros_attached_to_all_neighbours() {
        let (state, mut rx) = fresh_state();
        let peer_a = fresh_peer();
        let peer_b = fresh_peer();
        state.add_neighbour_transport(peer_a, [10; 8], ConnectionModule::Lan);
        state.add_neighbour_transport(peer_b, [20; 8], ConnectionModule::Lan);

        let user_id = [1; 8];
        install_user(&state, user_id, 7);
        state.user_dict.write().unwrap().bind(3, user_id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 3);

        tick_origin(&state, 0);

        let m1 = rx.try_recv().expect("outbound 1");
        let m2 = rx.try_recv().expect("outbound 2");
        assert!(rx.try_recv().is_err());

        let u1 = decode_frame(&m1.bytes);
        let u2 = decode_frame(&m2.bytes);
        assert_eq!(u1.user_mappings.len(), 1);
        assert_eq!(u2.user_mappings.len(), 1);
        assert_eq!(u1.user_mappings[0].target_id, user_id);
        assert_eq!(u2.user_mappings[0].target_id, user_id);
    }

    /// tick_origin increments seq_num by exactly one per invocation. The
    /// wire entry's `seq` equals the new value after the increment.
    #[test]
    fn tick_origin_wire_seq_matches_incremented_seq_num() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

        let before = state.seq_num.read().unwrap().value();
        tick_origin(&state, 0);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert_eq!(update.user_entries[0].seq, before.wrapping_add(1));
    }
}

// ---------- tick_relay ----------

mod relay {
    use super::*;
    use crate::router_v2::{
        codec::{messages::RoutingUpdate, Header, RoutingMessage},
        index::Space,
        propagation::tick_relay,
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
    };

    fn decode_frame(bytes: &[u8]) -> RoutingUpdate {
        let (header, body_slice) = Header::decode(bytes).expect("frame header");
        assert_eq!(header.message_type, RoutingMessage::RoutingUpdate);
        let payload = &body_slice[..header.payload_len as usize];
        RoutingUpdate::decode(payload).expect("routing-update body")
    }

    /// Installs a routing entry at `(space, own_idx)`, binds the own dict
    /// for the target, and pushes into the relay queue.
    fn queue_entry(
        state: &RouterV2State,
        space: Space,
        own_idx: u16,
        target: TargetRef,
        target_id: [u8; 8],
        next_hop_idx: u16,
        transport: ConnectionModule,
        seq: u16,
        metric: u16,
        local_only: bool,
    ) {
        bind_own_dict(state, space, own_idx, target_id);
        let arc = Arc::new(RwLock::new(RoutingEntry {
            target_index: own_idx,
            target,
            seq_num: SeqNum::from(seq),
            metric,
            next_hop: next_hop_idx,
            transport,
            last_update: 1_000,
            hop_count: 2,
            local_only,
        }));
        state
            .routing_table
            .write()
            .unwrap()
            .set(space, own_idx, arc);
        state.relay_queue.write().unwrap().insert((space, own_idx));
    }

    // ---------- empty cases ----------

    #[test]
    fn empty_queue_pushes_nothing() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

        tick_relay(&state, 5_000);

        assert!(rx.try_recv().is_err());
    }

    #[test]
    fn no_neighbours_pushes_nothing() {
        let (state, mut rx) = fresh_state();
        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            500,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        tick_relay(&state, 5_000);

        assert!(rx.try_recv().is_err());
    }

    // ---------- happy path ----------

    /// One queued entry, one neighbour → one outbound with the correct
    /// wire fields.
    #[test]
    fn queued_entry_routed_to_neighbour() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

        // Entry's next_hop resolves to a phantom neighbour (not the
        // outgoing one), so split-horizon allows.
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            501,
            ConnectionModule::Lan,
            3,
            20,
            false,
        );

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("one outbound");
        assert_eq!(msg.peer, peer);
        assert_eq!(msg.transport, ConnectionModule::Lan);

        let update = decode_frame(&msg.bytes);
        assert_eq!(update.user_entries.len(), 1);
        assert!(update.node_entries.is_empty());
        let wire = &update.user_entries[0];
        assert_eq!(wire.abs_idx, 5);
        assert_eq!(wire.seq, 3);
        assert_eq!(wire.metric, 20);
        assert_eq!(wire.hop_count, 2);
    }

    // ---------- split horizon ----------

    /// Split-horizon: the neighbour whose id equals the entry's resolved
    /// next_hop must NOT receive this entry. A second neighbour still does.
    #[test]
    fn split_horizon_blocks_return_to_source_neighbour() {
        let (state, mut rx) = fresh_state();

        let peer_source = fresh_peer();
        let peer_other = fresh_peer();
        state.add_neighbour_transport(peer_source, [77; 8], ConnectionModule::Lan);
        state.add_neighbour_transport(peer_other, [88; 8], ConnectionModule::Lan);

        // next_hop resolves to [77;8] — the source neighbour.
        bind_own_dict(&state, Space::Node, 500, [77; 8]);

        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            500,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        tick_relay(&state, 5_000);

        // Only peer_other should receive; peer_source is split-horizon blocked.
        let mut recipients = Vec::new();
        while let Ok(msg) = rx.try_recv() {
            recipients.push(msg.peer);
        }
        assert_eq!(recipients, vec![peer_other]);
    }

    // ---------- sphere filter ----------

    /// A user-target entry must not cross the Internet membrane (§2.3).
    #[test]
    fn sphere_filter_drops_user_target_on_internet_outgoing() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        // next_hop points at a different node so split-horizon allows.
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        tick_relay(&state, 5_000);

        // No outbound: the only survived entry would be user-space, which
        // gets sphere-filtered before send, and no intros exist to save
        // the batch. Empty-batch shortcut kicks in.
        assert!(rx.try_recv().is_err());
    }

    /// A gateway-node entry DOES cross the Internet membrane.
    #[test]
    fn sphere_filter_allows_gateway_node_on_internet_outgoing() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let gateway = install_node(&state, [9; 8], 0, true); // is_gateway = true
        queue_entry(
            &state,
            Space::Node,
            5,
            TargetRef::Node(gateway),
            [9; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("gateway entry must propagate");
        let update = decode_frame(&msg.bytes);
        assert_eq!(update.node_entries.len(), 1);
    }

    // ---------- local_only wire rewrite ----------

    /// Stored `local_only = true` → Internet-outgoing wire flag becomes
    /// `false` (§7.4 sender rule). Uses a gateway node so the entry
    /// survives the sphere filter.
    #[test]
    fn local_only_stripped_for_internet_outgoing() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let gateway = install_node(&state, [9; 8], 0, true);
        queue_entry(
            &state,
            Space::Node,
            5,
            TargetRef::Node(gateway),
            [9; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            true, // stored local_only
        );

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert!(
            !update.node_entries[0].local_only,
            "Internet-outgoing must strip local_only",
        );
    }

    /// Stored `local_only = true` → Local-outgoing wire flag equals the
    /// stored value (pass through).
    #[test]
    fn local_only_preserved_for_local_outgoing() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            true,
        );

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert!(update.user_entries[0].local_only);
    }

    // ---------- delta-encoding invariant ----------

    /// Wire entries must be sorted by abs_idx per space. HashSet iteration
    /// is non-deterministic, so this pins the sort.
    #[test]
    fn wire_entries_sorted_by_abs_idx() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        // Queue three entries at unsorted indices.
        for (i, own_idx) in [50u16, 10, 200].iter().enumerate() {
            let user = install_user(&state, [i as u8 + 1; 8], 0);
            queue_entry(
                &state,
                Space::User,
                *own_idx,
                TargetRef::User(user),
                [i as u8 + 1; 8],
                501,
                ConnectionModule::Lan,
                1,
                10,
                false,
            );
        }

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        let idxs: Vec<u16> = update.user_entries.iter().map(|e| e.abs_idx).collect();
        assert_eq!(idxs, vec![10, 50, 200]);
    }

    // ---------- introductions ----------

    /// Pending introductions must be attached to the outbound message
    /// alongside any surviving entries.
    #[test]
    fn pending_introductions_attached_to_message() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        // Set up an introduction.
        let intro_id = [11; 8];
        install_user(&state, intro_id, 3);
        state.user_dict.write().unwrap().bind(7, intro_id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 7);

        // Also queue a routing entry so the batch isn't empty on the
        // entry side (empty-batch shortcut wouldn't fire since intros
        // exist, but this exercises the mixed case).
        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert_eq!(update.user_mappings.len(), 1);
        assert_eq!(update.user_mappings[0].abs_idx, 7);
        assert_eq!(update.user_mappings[0].target_id, intro_id);
        assert_eq!(update.user_mappings[0].version, 3);
    }

    /// Introductions alone are enough to send — even with no entries.
    #[test]
    fn introductions_alone_produce_outbound() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);

        let intro_id = [11; 8];
        install_user(&state, intro_id, 3);
        state.user_dict.write().unwrap().bind(7, intro_id);
        state
            .reintroduction_tracker
            .write()
            .unwrap()
            .mark_first_time(Space::User, 7);

        tick_relay(&state, 5_000);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert_eq!(update.user_mappings.len(), 1);
        assert!(update.user_entries.is_empty());
    }

    // ---------- empty-batch shortcut ----------

    /// When every queued entry is filtered out AND no introductions
    /// exist, the tick must not emit anything for that neighbour.
    #[test]
    fn empty_batch_shortcut_suppresses_purely_empty_message() {
        let (state, mut rx) = fresh_state();

        // One neighbour, entry destined for split-horizon block.
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 500, [77; 8]);

        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            500, // → [77;8], split-horizon blocks
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        tick_relay(&state, 5_000);

        assert!(
            rx.try_recv().is_err(),
            "empty batch shortcut must suppress the send",
        );
    }

    // ---------- queue drain ----------

    #[test]
    fn relay_queue_drained_after_tick() {
        let (state, mut _rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let user = install_user(&state, [1; 8], 0);
        queue_entry(
            &state,
            Space::User,
            5,
            TargetRef::User(user),
            [1; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            false,
        );

        assert_eq!(state.relay_queue.read().unwrap().len(), 1);

        tick_relay(&state, 5_000);

        assert!(
            state.relay_queue.read().unwrap().is_empty(),
            "queue must be empty after tick",
        );
    }

    // ---------- multi-transport neighbour ----------

    /// A neighbour on both LAN and Internet receives two messages, one
    /// per transport, each with the correct local_only rewrite for its
    /// outgoing sphere. Uses a gateway node so the entry survives on both.
    #[test]
    fn multi_transport_neighbour_gets_one_message_per_transport() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        // next_hop points at a different node.
        bind_own_dict(&state, Space::Node, 501, [88; 8]);

        let gateway = install_node(&state, [9; 8], 0, true);
        queue_entry(
            &state,
            Space::Node,
            5,
            TargetRef::Node(gateway),
            [9; 8],
            501,
            ConnectionModule::Lan,
            1,
            10,
            true, // stored local_only
        );

        tick_relay(&state, 5_000);

        let mut by_transport: HashMap<ConnectionModule, bool> = HashMap::new();
        while let Ok(msg) = rx.try_recv() {
            let update = decode_frame(&msg.bytes);
            let wire_local_only = update.node_entries[0].local_only;
            by_transport.insert(msg.transport, wire_local_only);
        }
        assert_eq!(by_transport.len(), 2, "one message per transport");
        assert_eq!(
            by_transport[&ConnectionModule::Lan],
            true,
            "Local passes stored=true through"
        );
        assert_eq!(
            by_transport[&ConnectionModule::Internet],
            false,
            "Internet strips local_only regardless of stored",
        );
    }
}

// ---------- handle_node_manifest ----------

mod handle_node_manifest {
    use super::*;
    use crate::router_v2::{
        codec::messages::{ManifestEntry, NodeManifest},
        identity::{delegation_signing_input, Multikey},
        manifest::Manifest,
        table::{Node, User},
        test_utils::*,
    };
    use libp2p::identity::Keypair;

    fn keypair_and_multikey() -> (Keypair, Multikey) {
        let kp = Keypair::generate_ed25519();
        let mk = Multikey::from(kp.public());
        (kp, mk)
    }

    fn sign_entry(
        user_kp: &Keypair,
        host_mk: &Multikey,
        user_id: [u8; 8],
        timeout: u64,
    ) -> ManifestEntry {
        let signing_input = delegation_signing_input(&host_mk.encode(), timeout);
        let sig_bytes = user_kp.sign(&signing_input).unwrap();
        let entry_signature: [u8; 64] = sig_bytes.try_into().unwrap();
        ManifestEntry {
            user_id,
            timeout,
            entry_signature,
            profile_version: 0,
        }
    }

    /// Install a Node with a specific public key so we can sign
    /// matching messages. Returns the origin's node_id.
    fn install_origin_node(state: &RouterV2State, mk: &Multikey) -> [u8; 8] {
        let id = mk.to_id();
        let node = Node {
            id,
            public_key: Some(mk.clone()),
            manifest_version: 0,
            advertised_version: 0,
            is_gateway: false,
            delegated_users: Vec::new(),
            manifest_signature: None,
            retained_chunks: None,
            learn_sphere: None,
            manifest_log: crate::router_v2::manifest::ManifestLog::default(),
        };
        state.nodes.write().unwrap().insert(id, node);
        id
    }

    fn install_user_with_key(state: &RouterV2State, mk: &Multikey) -> [u8; 8] {
        let id = mk.to_id();
        let user = User {
            id,
            public_key: Some(mk.clone()),
            profile_version: 0,
            routing_entry: None,
            delegation_gateways: Vec::new(),
            is_hosted: false,
        };
        state.users.write().unwrap().insert(id, user);
        id
    }

    /// Wire a self-origin scenario: neighbour with origin's node_id,
    /// origin bound at reserved idx 0 in the neighbour's node mirror,
    /// origin's Node record installed with a real key.
    fn setup_self_origin(state: &RouterV2State, host_mk: &Multikey) -> (libp2p::PeerId, [u8; 8]) {
        let host_id = install_origin_node(state, host_mk);
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);
        // Origin uses RESERVED_INDEX 0 in the sender's frame (§3.2).
        state
            .mirrors
            .write()
            .unwrap()
            .get_mut(&peer)
            .unwrap()
            .nodes
            .bind(0, host_id);
        (peer, host_id)
    }

    fn build_signed_manifest(
        host_kp: &Keypair,
        host_mk: &Multikey,
        version: u32,
        is_gateway: bool,
        entries: Vec<ManifestEntry>,
    ) -> Vec<NodeManifest> {
        let mut manifest = Manifest::new();
        manifest.manifest_version = version;
        manifest.set_gateway(is_gateway);
        manifest.set_entries(entries);
        manifest
            .build_chunks(host_mk.to_id(), host_kp, &host_mk.encode())
            .unwrap()
    }

    // ---------- happy path ----------

    #[test]
    fn happy_path_commits_manifest_to_node_record() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, host_id) = setup_self_origin(&state, &host_mk);

        let (user_kp, user_mk) = keypair_and_multikey();
        let user_id = install_user_with_key(&state, &user_mk);

        let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)];
        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, entries);

        state
            .handle_node_manifest(
                chunks.into_iter().next().unwrap(),
                500,
                ConnectionModule::Lan,
            )
            .unwrap();

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&host_id).unwrap();
        let node = node_arc.read().unwrap();
        assert_eq!(node.manifest_version, 5);
        assert!(node.is_gateway);
        assert_eq!(node.delegated_users.len(), 1);
        assert_eq!(node.delegated_users[0].user_id, user_id);
        assert_eq!(node.delegated_users[0].delegation_timeout, 1_000_000);
    }

    // ---------- drop paths ----------
    //
    // Under the pull-based model (spec §8.5), NODE_MANIFEST is link-local
    // and carries origin_node_id directly on the wire — `handle_node_manifest`
    // takes no `neighbour` parameter. The old "unknown neighbour is noop"
    // test is therefore obsolete; the equivalent drop path (origin_node_id
    // maps to no Node record) is covered by `unknown_origin_node_id_is_noop`
    // below.

    /// Under the pull-based model (spec §8.5), NODE_MANIFEST carries
    /// `origin_node_id` on the wire — no index translation via the
    /// neighbour's mirror. If the id doesn't match any Node record we
    /// hold, the handler drops the message.
    #[test]
    fn unknown_origin_node_id_is_noop() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let host_id = install_origin_node(&state, &host_mk);
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);

        // Build a signed manifest, then rewrite origin_node_id to point
        // at a Node we have no record of.
        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
        let mut msg = chunks.into_iter().next().unwrap();
        msg.origin_node_id = [99; 8];

        state
            .handle_node_manifest(msg, 0, ConnectionModule::Lan)
            .unwrap();

        // The real origin's Node record is untouched.
        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&host_id)
                .unwrap()
                .read()
                .unwrap()
                .manifest_version,
            0,
        );
        // No stub was created for the unknown id.
        assert!(state.nodes.read().unwrap().get(&[99; 8]).is_none());
    }

    #[test]
    fn origin_with_no_public_key_is_noop() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let host_id = host_mk.to_id();

        // Install origin Node with NO public key.
        state.nodes.write().unwrap().insert(
            host_id,
            Node {
                id: host_id,
                public_key: None,
                manifest_version: 0,
                advertised_version: 0,
                is_gateway: false,
                delegated_users: Vec::new(),
                manifest_signature: None,
                retained_chunks: None,
                learn_sphere: None,
                manifest_log: crate::router_v2::manifest::ManifestLog::default(),
            },
        );

        let peer = fresh_peer();
        state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);
        state
            .mirrors
            .write()
            .unwrap()
            .get_mut(&peer)
            .unwrap()
            .nodes
            .bind(0, host_id);

        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
        state
            .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
            .unwrap();

        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&host_id)
                .unwrap()
                .read()
                .unwrap()
                .manifest_version,
            0,
        );
    }

    #[test]
    fn tampered_chunk_signature_dropped() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, host_id) = setup_self_origin(&state, &host_mk);

        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, vec![]);
        let mut msg = chunks.into_iter().next().unwrap();
        msg.manifest_signature[0] ^= 0xFF;

        state
            .handle_node_manifest(msg, 0, ConnectionModule::Lan)
            .unwrap();

        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&host_id)
                .unwrap()
                .read()
                .unwrap()
                .manifest_version,
            0,
        );
    }

    // ---------- per-entry filtering ----------

    /// One bad entry sig + one good → only the bad one filtered; the
    /// good one lands in the Node's delegated_users.
    #[test]
    fn bad_per_entry_signature_drops_only_that_entry() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, host_id) = setup_self_origin(&state, &host_mk);

        let (good_kp, good_mk) = keypair_and_multikey();
        let good_id = install_user_with_key(&state, &good_mk);
        let (bad_kp, bad_mk) = keypair_and_multikey();
        let bad_id = install_user_with_key(&state, &bad_mk);

        let good_entry = sign_entry(&good_kp, &host_mk, good_id, 1_000_000);
        let mut bad_entry = sign_entry(&bad_kp, &host_mk, bad_id, 1_000_000);
        bad_entry.entry_signature[0] ^= 0xFF;

        let chunks =
            build_signed_manifest(&host_kp, &host_mk, 1, false, vec![good_entry, bad_entry]);
        state
            .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
            .unwrap();

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&host_id).unwrap();
        let node = node_arc.read().unwrap();
        // §8.8 step 5: both entries are *stored* byte-exact — the manifest
        // signature covers the whole set, so dropping one would leave this node
        // unable to serve it or to apply a later delta against it.
        assert_eq!(node.delegated_users.len(), 2);

        // Verification gates *use*: only the well-signed entry earns a
        // delegation gateway.
        drop(node);
        drop(nodes);
        let users = state.users.read().unwrap();
        assert_eq!(
            users.get(&good_id).unwrap().read().unwrap().delegation_gateways.len(),
            1,
            "the correctly signed entry is trusted"
        );
        assert!(
            users.get(&bad_id).unwrap().read().unwrap().delegation_gateways.is_empty(),
            "a bad per-entry signature must not earn trust"
        );
    }

    #[test]
    fn expired_entry_dropped_at_receive_time() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, host_id) = setup_self_origin(&state, &host_mk);

        let (user_kp, user_mk) = keypair_and_multikey();
        let user_id = install_user_with_key(&state, &user_mk);

        // timeout=500, now=1000 → expired.
        let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 500)];
        let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, entries);
        state
            .handle_node_manifest(
                chunks.into_iter().next().unwrap(),
                1_000,
                ConnectionModule::Lan,
            )
            .unwrap();

        // Stored regardless — expiry is a trust judgement, not a storage one,
        // and the stored set must stay byte-identical to what was signed.
        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&host_id)
                .unwrap()
                .read()
                .unwrap()
                .delegated_users
                .len(),
            1,
        );

        // §10.4: an expired delegation is never trusted.
        assert!(
            state
                .users
                .read()
                .unwrap()
                .get(&user_id)
                .unwrap()
                .read()
                .unwrap()
                .delegation_gateways
                .is_empty(),
            "an expired delegation must not earn a gateway"
        );
    }

    // ---------- flag propagation ----------

    #[test]
    fn is_gateway_flag_reflected_in_node_record() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, host_id) = setup_self_origin(&state, &host_mk);

        let chunks = build_signed_manifest(&host_kp, &host_mk, 1, true, vec![]);
        state
            .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
            .unwrap();

        assert!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&host_id)
                .unwrap()
                .read()
                .unwrap()
                .is_gateway,
        );
    }

    /// An entry whose subject's key we do not hold is stored but not trusted.
    ///
    /// This is exactly why storage and trust are separated: the entry survives,
    /// so the manifest stays servable and a later delta still applies against
    /// it, and the entry becomes trusted the moment §11.5 delivers the profile
    /// — without re-fetching the manifest.
    #[test]
    fn entry_for_user_with_unknown_key_is_stored_but_untrusted() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (_, host_id) = setup_self_origin(&state, &host_mk);

        let (user_kp, user_mk) = keypair_and_multikey();
        let user_id = user_mk.to_id();
        // Do NOT install user — their key is unknown.

        let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)];
        let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, entries);
        state
            .handle_node_manifest(chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
            .unwrap();

        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&host_id)
                .unwrap()
                .read()
                .unwrap()
                .delegated_users
                .len(),
            1,
            "stored byte-exact even though unverifiable"
        );

        // A stub User record exists, but with no gateway — nothing vouches for
        // it yet.
        assert!(
            state
                .users
                .read()
                .unwrap()
                .get(&user_id)
                .unwrap()
                .read()
                .unwrap()
                .delegation_gateways
                .is_empty()
        );
    }
}

// ---------- on_neighbour_connect ----------

mod on_neighbour_connect {
    use super::*;
    use crate::router_v2::{
        codec::{messages::IndexDump, Header, RoutingMessage},
        index::Space,
        propagation::on_neighbour_connect,
        test_utils::*,
    };

    /// Decode a framed OutboundMsg body into an IndexDump.
    fn decode_dump_body(bytes: &[u8]) -> IndexDump {
        let (header, body_slice) = Header::decode(bytes).expect("frame header");
        assert_eq!(header.message_type, RoutingMessage::IndexDump);
        let payload = &body_slice[..header.payload_len as usize];
        IndexDump::decode(payload).expect("IndexDump body")
    }

    /// A node with nothing bound still emits a dump — the message itself is
    /// the signal that the neighbour should introduce itself back.
    ///
    /// Both sections are empty: §3.5 ties each reserved index to a propagation
    /// form, and a fresh node hosts no users and holds no INTERNET connection,
    /// so it is in user form with no hosted user yet. Neither space has a
    /// self-binding to advertise.
    #[test]
    fn empty_state_still_sends_an_empty_dump() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        on_neighbour_connect(&state, peer, ConnectionModule::Lan);

        let msg = rx.try_recv().expect("bootstrap must always emit");
        assert_eq!(msg.peer, peer);
        assert_eq!(msg.transport, ConnectionModule::Lan);
        let dump = decode_dump_body(&msg.bytes);

        assert!(dump.user_mappings.is_empty());
        assert!(
            dump.node_mappings.is_empty(),
            "no self-binding exists until a propagation form is established"
        );

        assert!(rx.try_recv().is_err(), "one dump per neighbour");
    }

    /// Once a hosted user takes the user-space reserved slot, the dump carries
    /// it — this is what lets a peer translate the origin's user entry at
    /// index 0.
    #[test]
    fn user_form_dump_carries_the_hosted_user_self_binding() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        let user_id = [42; 8];
        state.register_hosted_user(user_id, 7);

        on_neighbour_connect(&state, peer, ConnectionModule::Lan);

        let dump = decode_dump_body(&rx.try_recv().expect("one outbound").bytes);
        assert_eq!(dump.user_mappings.len(), 1);
        assert_eq!(dump.user_mappings[0].abs_idx, 0);
        assert_eq!(dump.user_mappings[0].target_id, user_id);
        assert_eq!(dump.user_mappings[0].version, 7);
        assert!(
            dump.node_mappings.is_empty(),
            "node-space reserved slot stays unbound in user form"
        );
    }

    #[test]
    fn populated_dicts_produce_correct_mappings() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        // Set up a user binding + user record with a specific version.
        let user_id = [1; 8];
        install_user(&state, user_id, 42);
        bind_own_dict(&state, Space::User, 7, user_id);

        // And a node binding + node record.
        let node_id = [2; 8];
        install_node(&state, node_id, 99, false);
        bind_own_dict(&state, Space::Node, 8, node_id);

        on_neighbour_connect(&state, peer, ConnectionModule::Lan);

        let msg = rx.try_recv().expect("one outbound");
        let dump = decode_dump_body(&msg.bytes);

        // User side.
        assert_eq!(dump.user_mappings.len(), 1);
        assert_eq!(dump.user_mappings[0].abs_idx, 7);
        assert_eq!(dump.user_mappings[0].target_id, user_id);
        assert_eq!(dump.user_mappings[0].version, 42);

        // Node side: only our installed binding at idx 8. The node-space
        // reserved slot is unbound in user form (§3.5), so there is no
        // self-mapping alongside it.
        assert_eq!(dump.node_mappings.len(), 1);
        let installed = &dump.node_mappings[0];
        assert_eq!(installed.abs_idx, 8);
        assert_eq!(installed.target_id, node_id);
        assert_eq!(installed.version, 99);
    }

    /// Delta-encoding invariant: mappings must arrive sorted by abs_idx
    /// on the wire. HashMap iteration is non-deterministic; the sort in
    /// the code is what pins this contract.
    #[test]
    fn mappings_sorted_by_abs_idx() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        // Bind at unsorted indices.
        for (i, idx) in [100u16, 5, 50].iter().enumerate() {
            let id = [i as u8 + 1; 8];
            install_user(&state, id, 0);
            bind_own_dict(&state, Space::User, *idx, id);
        }

        on_neighbour_connect(&state, peer, ConnectionModule::Lan);

        let msg = rx.try_recv().expect("one outbound");
        let dump = decode_dump_body(&msg.bytes);
        let idxs: Vec<u16> = dump.user_mappings.iter().map(|m| m.abs_idx).collect();
        assert_eq!(idxs, vec![5, 50, 100]);
    }

    #[test]
    fn ble1m_transport_skips_send() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        // Populate dicts so we'd otherwise have something to send.
        install_user(&state, [1; 8], 0);
        bind_own_dict(&state, Space::User, 7, [1; 8]);

        on_neighbour_connect(&state, peer, ConnectionModule::Ble1m);

        assert!(rx.try_recv().is_err(), "BLE must not receive INDEX_DUMP");
    }

    #[test]
    fn ble_coded_transport_skips_send() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        install_user(&state, [1; 8], 0);
        bind_own_dict(&state, Space::User, 7, [1; 8]);

        on_neighbour_connect(&state, peer, ConnectionModule::BleCoded);

        assert!(
            rx.try_recv().is_err(),
            "BLE-coded must not receive INDEX_DUMP"
        );
    }

    /// dict has an idx→id binding but no matching User record — the
    /// `unwrap_or(0)` fallback surfaces here.
    #[test]
    fn missing_user_record_defaults_version_to_zero() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        // Bind but don't install the User.
        bind_own_dict(&state, Space::User, 5, [77; 8]);

        on_neighbour_connect(&state, peer, ConnectionModule::Lan);

        let msg = rx.try_recv().expect("one outbound");
        let dump = decode_dump_body(&msg.bytes);
        assert_eq!(dump.user_mappings.len(), 1);
        assert_eq!(dump.user_mappings[0].abs_idx, 5);
        assert_eq!(dump.user_mappings[0].target_id, [77; 8]);
        assert_eq!(dump.user_mappings[0].version, 0);
    }

    #[test]
    fn emits_indexdump_message_type() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();

        on_neighbour_connect(&state, peer, ConnectionModule::Lan);

        let msg = rx.try_recv().expect("one outbound");
        let (header, _) = Header::decode(&msg.bytes).expect("frame header");
        assert_eq!(header.message_type, RoutingMessage::IndexDump);
    }
}

// ---------- self-delegation & version bumps (§10.3, §10.8) ----------

mod self_delegation {
    use super::*;
    use crate::{
        node::user_accounts::UserAccount,
        router_v2::{
            manifest::{LogRecord, Manifest},
            test_utils::*,
            BumpTrigger,
        },
        storage::manifest_state::HostManifestState,
    };
    use libp2p::{identity::Keypair, PeerId};

    /// `manifest_rate_limit` defaults to 60 s (§14); the window is in ms.
    const WINDOW_MS: u64 = 60_000;
    const TTL_MS: u64 = 6 * 60 * 60 * 1000;

    fn fresh_account() -> UserAccount {
        let keys = Keypair::generate_ed25519();
        UserAccount {
            id: PeerId::from(keys.public()),
            keys,
            name: "test".into(),
            password_hash: None,
            password_salt: None,
        }
    }

    /// Registers `account` as a hosted user with a self-delegation, the way
    /// `UserAccounts::create` does. Returns its routing id.
    fn delegate(state: &RouterV2State, account: &UserAccount, now_ms: u64) -> [u8; 8] {
        let id = account.routing_user_id();
        let delegation = account.issue_self_delegation(&state.host_mk, now_ms + TTL_MS);
        state.add_self_delegation(id, 0, delegation);
        id
    }

    // ----- the signing boundary -----

    /// The round trip that proves the split works: libqaul signs with the
    /// user's key, router_v2 stores the artefact, and a receiver reconstructs
    /// the same input from `(host_mk, entry.timeout)` and verifies it against
    /// the user's multikey. If the signing input ever drifts on either side,
    /// this is the test that fails.
    #[test]
    fn stored_delegation_verifies_against_the_users_key() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let id = delegate(&state, &account, 0);

        let manifest = state.manifest.read().unwrap();
        let entry = manifest
            .entries()
            .iter()
            .find(|e| e.user_id == id)
            .expect("entry stored");

        assert!(
            Manifest::verify_entry(entry, &state.host_mk, &account.multikey()).is_ok(),
            "the stored entry must verify against the delegating user's key"
        );
    }

    /// The timeout is signed content (§10.1), so extending an entry's life
    /// without a fresh signature must not verify.
    #[test]
    fn tampering_with_the_timeout_breaks_verification() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let id = delegate(&state, &account, 0);

        let mut entry = *state
            .manifest
            .read()
            .unwrap()
            .entries()
            .iter()
            .find(|e| e.user_id == id)
            .unwrap();
        entry.timeout = entry.timeout.saturating_add(1);

        assert!(
            Manifest::verify_entry(&entry, &state.host_mk, &account.multikey()).is_err(),
            "an altered timeout must invalidate the delegation"
        );
    }

    /// A delegation is bound to one host. Another host's key must not verify
    /// it, or a node could claim to represent users that never authorised it.
    #[test]
    fn a_delegation_does_not_verify_for_a_different_host() {
        let (state, _rx) = fresh_state();
        let (other_host, _rx2) = fresh_state();
        let account = fresh_account();
        let id = delegate(&state, &account, 0);

        let manifest = state.manifest.read().unwrap();
        let entry = manifest.entries().iter().find(|e| e.user_id == id).unwrap();

        assert!(
            Manifest::verify_entry(entry, &other_host.host_mk, &account.multikey()).is_err(),
            "the signature binds the authorisation to one specific host"
        );
    }

    // ----- accumulate -----

    /// §10.8: a change marks the manifest dirty but must not bump on its own —
    /// timing belongs to the bump.
    #[test]
    fn adding_a_delegation_marks_dirty_without_bumping() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let id = delegate(&state, &account, 0);

        assert_eq!(state.manifest.read().unwrap().entries().len(), 1);
        assert_eq!(
            state.manifest.read().unwrap().manifest_version,
            0,
            "the add itself must not bump"
        );
        assert!(state.dirty_delegations.read().unwrap().contains(&id));
    }

    #[test]
    fn accumulated_bump_inside_the_window_is_declined() {
        let (state, _rx) = fresh_state();
        delegate(&state, &fresh_account(), 0);

        assert_eq!(
            state.try_bump_manifest_version(1_000, BumpTrigger::Accumulated),
            None,
            "still inside the 60s window"
        );
        assert_eq!(state.manifest.read().unwrap().manifest_version, 0);
    }

    #[test]
    fn accumulated_bump_with_nothing_dirty_is_declined() {
        let (state, _rx) = fresh_state();

        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS * 10, BumpTrigger::Accumulated),
            None,
            "no change to fold"
        );
    }

    #[test]
    fn accumulated_bump_after_the_window_folds_and_logs() {
        let (state, _rx) = fresh_state();
        let id = delegate(&state, &fresh_account(), 0);

        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
            Some(1)
        );
        assert_eq!(state.manifest.read().unwrap().manifest_version, 1);
        assert!(
            state.dirty_delegations.read().unwrap().is_empty(),
            "the dirty set is consumed by the fold"
        );

        let records = state.own_manifest_log.read().unwrap().records_after(0);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].record_version(), 1);
        assert_eq!(records[0].user_id(), id);
        assert!(matches!(records[0], LogRecord::Add { .. }));
    }

    /// The core of §10.8: "changes that occur within a window accumulate and
    /// fold into a single bump". Two adds produce one version carrying two
    /// records, not two versions.
    #[test]
    fn two_adds_in_one_window_fold_into_a_single_bump() {
        let (state, _rx) = fresh_state();
        let a = delegate(&state, &fresh_account(), 0);
        let b = delegate(&state, &fresh_account(), 1_000);

        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
            Some(1),
            "both changes fold into one version"
        );

        let records = state.own_manifest_log.read().unwrap().records_after(0);
        assert_eq!(records.len(), 2);
        assert!(records.iter().all(|r| r.record_version() == 1));

        let mut ids: Vec<[u8; 8]> = records.iter().map(|r| r.user_id()).collect();
        ids.sort();
        let mut expected = vec![a, b];
        expected.sort();
        assert_eq!(ids, expected);
    }

    /// Folding from *current state* rather than replaying operations is what
    /// makes this correct for free: a user added and removed inside one window
    /// never appeared in a committed version, and collapses to one tombstone.
    #[test]
    fn add_then_remove_in_one_window_folds_to_a_tombstone() {
        let (state, _rx) = fresh_state();
        let id = delegate(&state, &fresh_account(), 0);
        assert!(state.remove_self_delegation(&id));

        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
            Some(1)
        );

        assert!(state.manifest.read().unwrap().entries().is_empty());
        let records = state.own_manifest_log.read().unwrap().records_after(0);
        assert_eq!(records.len(), 1);
        assert!(matches!(records[0], LogRecord::Tombstone { .. }));
    }

    /// Re-issuing an identical delegation is not a change. Without this the
    /// TTL refresh (§10.3) would bump the version every cycle forever and make
    /// every peer re-pull each time.
    #[test]
    fn identical_redelegation_is_not_a_change() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let id = account.routing_user_id();
        let delegation = account.issue_self_delegation(&state.host_mk, TTL_MS);

        assert!(state.add_self_delegation(id, 0, delegation));
        assert!(
            !state.add_self_delegation(id, 0, delegation),
            "an identical re-issue must report no change"
        );
        assert_eq!(state.manifest.read().unwrap().entries().len(), 1);
    }

    /// A fresh timeout means a fresh signature, which *is* a change.
    #[test]
    fn redelegating_with_a_new_timeout_is_a_change() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let id = account.routing_user_id();

        assert!(state.add_self_delegation(id, 0, account.issue_self_delegation(&state.host_mk, TTL_MS)));
        assert!(state.add_self_delegation(
            id,
            0,
            account.issue_self_delegation(&state.host_mk, TTL_MS + 1)
        ));
        assert_eq!(state.manifest.read().unwrap().entries().len(), 1);
    }

    #[test]
    fn removing_an_absent_delegation_is_not_a_change() {
        let (state, _rx) = fresh_state();
        assert!(!state.remove_self_delegation(&[9; 8]));
        assert!(state.dirty_delegations.read().unwrap().is_empty());
    }

    // ----- rate-limit bypasses -----

    /// §10.8 names the single↔multi transition as a bypass, and it is a trigger
    /// in its own right — so it must bump inside the window *and* with nothing
    /// dirty. A plain `force: bool` would get the second half wrong.
    #[test]
    fn form_transition_bumps_inside_the_window_with_nothing_dirty() {
        let (state, _rx) = fresh_state();

        assert_eq!(
            state.try_bump_manifest_version(0, BumpTrigger::FormTransition),
            Some(1)
        );
    }

    #[test]
    fn forced_removal_bypasses_the_window() {
        let (state, _rx) = fresh_state();
        let id = delegate(&state, &fresh_account(), 0);
        state.remove_self_delegation(&id);

        assert_eq!(
            state.try_bump_manifest_version(1_000, BumpTrigger::ForcedRemoval),
            Some(1),
            "§10.7 removal takes effect in the next relay batch"
        );
    }

    /// A bump restarts the window, so a second accumulated change has to wait
    /// out a fresh 60 s rather than riding the first bump's timestamp.
    #[test]
    fn a_bump_restarts_the_rate_limit_window() {
        let (state, _rx) = fresh_state();
        delegate(&state, &fresh_account(), 0);
        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated),
            Some(1)
        );

        delegate(&state, &fresh_account(), WINDOW_MS + 1);
        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS + 1_000, BumpTrigger::Accumulated),
            None,
            "the window restarted at the previous bump"
        );
        assert_eq!(
            state.try_bump_manifest_version(WINDOW_MS * 2, BumpTrigger::Accumulated),
            Some(2)
        );
    }

    // ----- persistence (§10.8 SHALL) -----

    /// §10.8: "An origin SHALL persist its `manifest_version` across restarts
    /// and resume from the persisted value… a regression would corrupt delta
    /// selection." This covers the snapshot/restore round trip without touching
    /// the filesystem.
    #[test]
    fn snapshot_round_trips_version_and_entries() {
        let (state, _rx) = fresh_state();
        let account = fresh_account();
        let id = delegate(&state, &account, 0);
        state.try_bump_manifest_version(WINDOW_MS, BumpTrigger::Accumulated);

        let snapshot = state.host_manifest_snapshot();
        assert_eq!(snapshot.manifest_version, 1);
        assert_eq!(snapshot.entries.len(), 1);
        assert_eq!(snapshot.entries[0].entry_signature.len(), 64);

        // A restarted node restores from it.
        let (restarted, _rx2) = fresh_state();
        restarted.restore_host_manifest(&snapshot);

        let manifest = restarted.manifest.read().unwrap();
        assert_eq!(manifest.manifest_version, 1, "version must not regress");
        assert_eq!(manifest.entries().len(), 1);
        assert_eq!(manifest.entries()[0].user_id, id);
    }

    /// A first startup with no file on disk must not look like a regression.
    #[test]
    fn default_host_state_restores_as_a_clean_origin() {
        let (state, _rx) = fresh_state();
        state.restore_host_manifest(&HostManifestState::default());

        assert_eq!(state.manifest.read().unwrap().manifest_version, 0);
        assert!(state.manifest.read().unwrap().entries().is_empty());
    }
}
