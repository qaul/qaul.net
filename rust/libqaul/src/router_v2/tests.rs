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
        let state = fresh_state();
        assert_eq!(state.next_hop_for_user([99; 8]), None);
    }

    #[test]
    fn known_user_with_no_routing_data_returns_none() {
        let state = fresh_state();
        install_user(&state, [1; 8], 0);
        assert_eq!(state.next_hop_for_user([1; 8]), None);
    }

    /// Step 2: a direct routing entry whose next_hop resolves through the
    /// dictionary should produce that hop's node id and the entry's transport.
    #[test]
    fn direct_routing_entry_resolves_next_hop_and_transport() {
        let state = fresh_state();
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
        let state = fresh_state();
        let user = install_user(&state, [1; 8], 0);

        let g_hi = install_node(&state, [10; 8], 0, true);
        let g_lo = install_node(&state, [20; 8], 0, true);

        bind_own_dict(&state, Space::Node, 50, [10; 8]);
        bind_own_dict(&state, Space::Node, 60, [20; 8]);

        let nbr_hi = [11; 8];
        let nbr_lo = [21; 8];
        bind_own_dict(&state, Space::Node, 101, nbr_hi);
        bind_own_dict(&state, Space::Node, 102, nbr_lo);

        let e_hi = make_entry(TargetRef::Node(g_hi.clone()), 101, 30, ConnectionModule::Lan);
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
            is_gateway: true,
            delegated_users: Vec::new(),
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
        let peer = fresh_peer();
        let err = state.translate_incoming(peer, Space::User, 5).unwrap_err();
        assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
    }

    #[test]
    fn translate_incoming_known_neighbour_unknown_idx_returns_unknown_mapping() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let err = state.translate_incoming(peer, Space::User, 5).unwrap_err();
        assert!(matches!(err, RoutingV2Error::UnknownMapping(5)));
    }

    /// If our own dict already has a binding for the ID, return the
    /// existing own_idx; do not allocate, do not mark the tracker.
    #[test]
    fn translate_incoming_existing_own_binding_returns_existing_idx() {
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [13; 8];
        bind_mirror(&state, peer, Space::User, 5, id);

        let first = state.translate_incoming(peer, Space::User, 5).unwrap();
        let second = state.translate_incoming(peer, Space::User, 5).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn translate_incoming_spaces_are_independent() {
        let state = fresh_state();
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
        let state = fresh_state();
        assert!(state.pending_introductions(Space::User).is_empty());
        assert!(state.pending_introductions(Space::Node).is_empty());
    }

    #[test]
    fn pending_introductions_returns_marked_user_with_correct_version() {
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();

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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();

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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        assert_eq!(n.manifest_version, 99);
        assert!(!n.is_gateway, "stub node is not a gateway by default");
        assert!(n.public_key.is_none());
    }

    #[test]
    fn apply_mapping_same_id_updates_version_only() {
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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
        let state = fresh_state();
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

    #[test]
    fn apply_mapping_incoming_version_fresher_updates_node() {
        let state = fresh_state();
        let peer = add_neighbour(&state);
        let id = [6; 8];
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

        assert_eq!(
            state
                .nodes
                .read()
                .unwrap()
                .get(&id)
                .unwrap()
                .read()
                .unwrap()
                .manifest_version,
            12,
        );
    }

    #[test]
    fn apply_mapping_user_and_node_spaces_are_independent() {
        let state = fresh_state();
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
