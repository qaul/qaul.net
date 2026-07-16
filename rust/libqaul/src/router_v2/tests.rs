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
        assert_eq!(n.manifest_version, 99);
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

    #[test]
    fn apply_mapping_incoming_version_fresher_updates_node() {
        let (state, _rx) = fresh_state();
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
        codec::messages::Entry,
        index::Space,
        seq::SeqNum,
        table::{RoutingEntry, TargetRef},
        test_utils::*,
    };
    use libp2p::PeerId;

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
        bind_own_dict(state, Space::Node, NEIGHBOUR_IDX_IN_NODE_DICT, NEIGHBOUR_NODE_ID);
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
        bind_own_dict(state, Space::Node, NEIGHBOUR_IDX_IN_NODE_DICT, NEIGHBOUR_NODE_ID);
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
        state.routing_table.write().unwrap().set(space, own_idx, entry);
    }

    fn wire_entry(
        abs_idx: u16,
        seq: u16,
        metric: u16,
        hop_count: u8,
        local_only: bool,
    ) -> Entry {
        Entry {
            abs_idx,
            seq,
            metric,
            hop_count,
            local_only,
        }
    }

    // ---------- TTL / drops ----------

    #[test]
    fn ttl_drop_when_incoming_hop_count_is_63() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, _) = setup_user_target(&state, 5, 42, target_id);

        state
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 1, 10, 63, false),
                1_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 1, 10, 62, false),
                1_000,
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
        bind_own_dict(&state, Space::Node, NEIGHBOUR_IDX_IN_NODE_DICT, NEIGHBOUR_NODE_ID);

        state
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 1, 10, 1, false),
                1_000,
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
        bind_own_dict(&state, Space::Node, NEIGHBOUR_IDX_IN_NODE_DICT, NEIGHBOUR_NODE_ID);
        // NOTE: no install_user — the record is missing.

        state
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 1, 10, 1, false),
                1_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 1, 10, 1, false),
                1_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 7, 10, 2, false),
                1_234,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::Node,
                wire_entry(5, 1, 10, 0, false),
                500,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 20, 10, 1, false),
                2_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 30, 10, 1, false),
                2_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 40, 10, 1, false),
                2_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 10, 5, 1, false),
                2_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 10, 10, 5, false),
                2_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 10, 30, 1, false),
                2_000,
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
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 20, 10, 1, true),
                2_000,
            )
            .unwrap();

        assert!(!state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .local_only);
    }

    /// Transitions to false: stored=true is overridden by incoming=false.
    #[test]
    fn local_only_transitions_to_false_when_incoming_is_false() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, user) = setup_user_target(&state, 5, 42, target_id);
        preload_entry(&state, Space::User, 42, TargetRef::User(user), 10, 50, true);

        state
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 20, 10, 1, false),
                2_000,
            )
            .unwrap();

        assert!(!state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .local_only);
    }

    #[test]
    fn local_only_empty_slot_uses_incoming_value() {
        let (state, _rx) = fresh_state();
        let target_id = [1; 8];
        let (peer, _) = setup_user_target(&state, 5, 42, target_id);

        state
            .apply_entry(
                peer,
                ConnectionModule::Lan,
                None,
                Space::User,
                wire_entry(5, 1, 10, 1, true),
                1_000,
            )
            .unwrap();

        assert!(state
            .routing_table
            .read()
            .unwrap()
            .get(Space::User, 42)
            .unwrap()
            .read()
            .unwrap()
            .local_only);
    }
}

// ---------- handle_routing_update ----------

mod handle_routing_update {
    use super::*;
    use crate::router_v2::{
        codec::messages::{Entry, Mapping, RoutingUpdate},
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
        bind_own_dict(state, Space::Node, NEIGHBOUR_IDX_IN_NODE_DICT, NEIGHBOUR_NODE_ID);
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
            .handle_routing_update(
                peer,
                ConnectionModule::Lan,
                None,
                empty_update(),
                1_000,
            )
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
            user_entries: vec![Entry {
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
        assert_eq!(
            mirrors.get(&peer).unwrap().users.id_of(5),
            Some(target_id),
        );
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
            user_entries: vec![Entry {
                abs_idx: 5,
                seq: 1,
                metric: 10,
                hop_count: 1,
                local_only: false,
            }],
            node_entries: vec![Entry {
                abs_idx: 6,
                seq: 1,
                metric: 15,
                hop_count: 1,
                local_only: false,
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

// ---------- received ----------

mod received {
    use super::*;
    use crate::router_v2::{
        codec::{
            messages::{Entry, Mapping, RoutingUpdate},
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
        bind_own_dict(state, Space::Node, NEIGHBOUR_IDX_IN_NODE_DICT, NEIGHBOUR_NODE_ID);
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
            user_entries: vec![Entry {
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

    /// Unimplemented message types (IndexDump, NodeManifest, ManifestDelta)
    /// must be skipped past — buf advances, next message still processes.
    #[test]
    fn unimplemented_message_type_is_skipped_and_next_processed() {
        let (state, _rx) = fresh_state();
        let peer = setup_neighbour(&state);
        let target_id = [4; 8];

        // Send an IndexDump with a small body, then a valid RoutingUpdate.
        let index_dump_body = [0x00u8; 2]; // arbitrary bytes; payload isn't decoded
        let mut bytes = frame(RoutingMessage::IndexDump, &index_dump_body);
        bytes.extend(frame_routing_update(&small_valid_update(target_id)));

        state
            .received(peer, ConnectionModule::Lan, None, &bytes, 1_000)
            .unwrap();

        // The IndexDump was skipped, then the RoutingUpdate applied.
        assert!(state.users.read().unwrap().get(&target_id).is_some());
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
        let entry = make_entry(TargetRef::User(target), 0, ConnectionModule::Internet, false);
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
        let entry = make_entry(TargetRef::User(target), 0, ConnectionModule::Internet, false);
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
        tick_origin(&state);
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

        tick_origin(&state);

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
    #[test]
    fn tick_origin_one_internet_neighbour_pushes_message_with_local_only_false() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        tick_origin(&state);

        let msg = rx.try_recv().expect("one outbound");
        let update = decode_frame(&msg.bytes);
        assert!(!update.user_entries[0].local_only);
    }

    /// A neighbour reachable on two transports gets *two* outbound
    /// messages this tick — one per (peer, transport) pair (§4.2).
    #[test]
    fn tick_origin_multi_transport_neighbour_pushes_one_per_transport() {
        let (state, mut rx) = fresh_state();
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Lan);
        state.add_neighbour_transport(peer, [77; 8], ConnectionModule::Internet);

        tick_origin(&state);

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

        tick_origin(&state);

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

        tick_origin(&state);

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
        tick_origin(&state);

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
        state.routing_table.write().unwrap().set(space, own_idx, arc);
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
        assert_eq!(by_transport[&ConnectionModule::Lan], true, "Local passes stored=true through");
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
        ManifestEntry { user_id, timeout, entry_signature }
    }

    /// Install a Node with a specific public key so we can sign
    /// matching messages. Returns the origin's node_id.
    fn install_origin_node(state: &RouterV2State, mk: &Multikey) -> [u8; 8] {
        let id = mk.to_id();
        let node = Node {
            id,
            public_key: Some(mk.clone()),
            manifest_version: 0,
            is_gateway: false,
            delegated_users: Vec::new(),
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
        };
        state.users.write().unwrap().insert(id, user);
        id
    }

    /// Wire a self-origin scenario: neighbour with origin's node_id,
    /// origin bound at reserved idx 0 in the neighbour's node mirror,
    /// origin's Node record installed with a real key.
    fn setup_self_origin(
        state: &RouterV2State,
        host_mk: &Multikey,
    ) -> (libp2p::PeerId, [u8; 8]) {
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
            .build_chunks(0, host_kp, &host_mk.encode())
            .unwrap()
    }

    // ---------- happy path ----------

    #[test]
    fn happy_path_commits_manifest_to_node_record() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (peer, host_id) = setup_self_origin(&state, &host_mk);

        let (user_kp, user_mk) = keypair_and_multikey();
        let user_id = install_user_with_key(&state, &user_mk);

        let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)];
        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, entries);

        state
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 500, ConnectionModule::Lan)
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

    #[test]
    fn unknown_neighbour_is_noop() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let host_id = install_origin_node(&state, &host_mk);
        let peer = fresh_peer(); // never added

        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
        state
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
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
    fn unknown_origin_index_in_mirror_is_noop() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let host_id = install_origin_node(&state, &host_mk);
        let peer = fresh_peer();
        state.add_neighbour_transport(peer, host_id, ConnectionModule::Lan);
        // Origin NOT bound in the mirror.

        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, false, vec![]);
        let mut msg = chunks.into_iter().next().unwrap();
        msg.origin_node_index = 42;

        state.handle_node_manifest(peer, msg, 0, ConnectionModule::Lan).unwrap();

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
                is_gateway: false,
                delegated_users: Vec::new(),
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
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
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
        let (peer, host_id) = setup_self_origin(&state, &host_mk);

        let chunks = build_signed_manifest(&host_kp, &host_mk, 5, true, vec![]);
        let mut msg = chunks.into_iter().next().unwrap();
        msg.manifest_signature[0] ^= 0xFF;

        state.handle_node_manifest(peer, msg, 0, ConnectionModule::Lan).unwrap();

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
        let (peer, host_id) = setup_self_origin(&state, &host_mk);

        let (good_kp, good_mk) = keypair_and_multikey();
        let good_id = install_user_with_key(&state, &good_mk);
        let (bad_kp, bad_mk) = keypair_and_multikey();
        let bad_id = install_user_with_key(&state, &bad_mk);

        let good_entry = sign_entry(&good_kp, &host_mk, good_id, 1_000_000);
        let mut bad_entry = sign_entry(&bad_kp, &host_mk, bad_id, 1_000_000);
        bad_entry.entry_signature[0] ^= 0xFF;

        let chunks = build_signed_manifest(
            &host_kp,
            &host_mk,
            1,
            false,
            vec![good_entry, bad_entry],
        );
        state
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
            .unwrap();

        let nodes = state.nodes.read().unwrap();
        let node_arc = nodes.get(&host_id).unwrap();
        let node = node_arc.read().unwrap();
        assert_eq!(node.delegated_users.len(), 1);
        assert_eq!(node.delegated_users[0].user_id, good_id);
    }

    #[test]
    fn expired_entry_dropped_at_receive_time() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (peer, host_id) = setup_self_origin(&state, &host_mk);

        let (user_kp, user_mk) = keypair_and_multikey();
        let user_id = install_user_with_key(&state, &user_mk);

        // timeout=500, now=1000 → expired.
        let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 500)];
        let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, entries);
        state
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 1_000, ConnectionModule::Lan)
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
            0,
        );
    }

    // ---------- flag propagation ----------

    #[test]
    fn is_gateway_flag_reflected_in_node_record() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (peer, host_id) = setup_self_origin(&state, &host_mk);

        let chunks = build_signed_manifest(&host_kp, &host_mk, 1, true, vec![]);
        state
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
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

    /// Documents current "no key → drop entry" behaviour. §11.5
    /// ProfileFetch (Phase 12) will change this to fetch-then-verify.
    #[test]
    fn entry_for_user_with_unknown_key_is_dropped() {
        let (state, mut _rx) = fresh_state();
        let (host_kp, host_mk) = keypair_and_multikey();
        let (peer, host_id) = setup_self_origin(&state, &host_mk);

        let (user_kp, user_mk) = keypair_and_multikey();
        let user_id = user_mk.to_id();
        // Do NOT install user — their key is unknown.

        let entries = vec![sign_entry(&user_kp, &host_mk, user_id, 1_000_000)];
        let chunks = build_signed_manifest(&host_kp, &host_mk, 1, false, entries);
        state
            .handle_node_manifest(peer, chunks.into_iter().next().unwrap(), 0, ConnectionModule::Lan)
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
            0,
        );
    }
}
