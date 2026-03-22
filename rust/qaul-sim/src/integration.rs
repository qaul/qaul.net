//! Integration tests for instance-based State structs.
//!
//! Verifies that all `*State` structs from libqaul can be created independently
//! and composed into a full instance without global state.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use libqaul::connections::ConnectionsState;
    use libqaul::router::RouterState;
    use libqaul::rpc::authentication::AuthenticationState;
    use libqaul::rpc::sys::SysRpcState;
    use libqaul::rpc::RpcState;
    use libqaul::services::ServicesState;
    use libqaul::storage::configuration::{Configuration, ConfigurationState, RoutingOptions};
    use libqaul::storage::database::DatabaseState;
    use libqaul::utilities::filelogger::FileLoggerState;

    /// Helper: build a Configuration without touching global state.
    /// `Configuration::default()` calls `get_default_config()` which requires
    /// the DEFCONFIGS global to be set. We construct one manually instead.
    fn test_config() -> Configuration {
        use libqaul::storage::configuration::*;
        Configuration {
            node: Node::default(),
            lan: Lan::default(),
            internet: Internet {
                active: true,
                peers: vec![],
                do_listen: false,
                listen: vec![],
            },
            user_accounts: Vec::new(),
            debug: DebugOption::default(),
            routing: RoutingOptions::default(),
        }
    }

    /// Verify all State structs can be independently created.
    #[test]
    fn all_state_structs_constructible() {
        let _config = ConfigurationState::from_config(test_config());
        let _db = DatabaseState::new_temporary();
        let _router = RouterState::new(RoutingOptions::default());
        let _services = ServicesState::new();
        let _connections = ConnectionsState::new();
        let _rpc = RpcState::new();
        let _sys_rpc = SysRpcState::new();
        let _auth = AuthenticationState::new();
        let _file_logger = FileLoggerState::new();
    }

    /// Verify ConfigurationState can hold custom configuration.
    #[test]
    fn configuration_state_from_config() {
        let mut config = test_config();
        config.routing.hop_count_penalty = 42;
        let state = ConfigurationState::from_config(config);
        let inner = state.inner.read().unwrap();
        assert_eq!(inner.routing.hop_count_penalty, 42);
    }

    /// Verify RPC channels work independently per instance.
    #[test]
    fn rpc_channels_independent() {
        let rpc1 = RpcState::new();
        let rpc2 = RpcState::new();

        // Send on rpc1
        rpc1.extern_send.send(vec![1, 2, 3]).unwrap();
        // rpc2 should have nothing
        assert!(rpc2.libqaul_receive.try_recv().is_err());
        // rpc1 should receive it
        let msg = rpc1.libqaul_receive.try_recv().unwrap();
        assert_eq!(msg, vec![1, 2, 3]);
    }

    /// Verify SYS RPC channels work independently per instance.
    #[test]
    fn sys_rpc_channels_independent() {
        let sys1 = SysRpcState::new();
        let sys2 = SysRpcState::new();

        sys1.extern_send.send(vec![4, 5, 6]).unwrap();
        assert!(sys2.libqaul_receive.try_recv().is_err());
        let msg = sys1.libqaul_receive.try_recv().unwrap();
        assert_eq!(msg, vec![4, 5, 6]);
    }

    /// Verify multiple RouterState instances are fully independent.
    #[test]
    fn multiple_router_states_independent() {
        use libp2p::identity::Keypair;
        use libqaul::connections::ConnectionModule;

        let config = RoutingOptions::default();
        let router1 = Arc::new(RouterState::new(config.clone()));
        let router2 = Arc::new(RouterState::new(config));

        let kp1 = Keypair::generate_ed25519();
        let kp2 = Keypair::generate_ed25519();
        let peer1 = kp1.public().to_peer_id();
        let peer2 = kp2.public().to_peer_id();

        // Register local users
        router1.connections.add_local_user(peer1, peer1);
        router2.connections.add_local_user(peer2, peer2);

        // Add a neighbour to router1 only
        router1.neighbours.update_node(ConnectionModule::Lan, peer2, 5000);

        // router1 should see peer2 as a LAN neighbour
        assert_eq!(
            router1.neighbours.is_neighbour(&peer2),
            ConnectionModule::Lan,
            "Router1 should see peer2 as LAN neighbour"
        );

        // router2 should NOT see peer1 as a neighbour
        assert_eq!(
            router2.neighbours.is_neighbour(&peer1),
            ConnectionModule::None,
            "Router2 should not see Router1's neighbours"
        );
    }

    /// Compose a "full node" from independent State structs.
    #[test]
    fn compose_full_node() {
        let config = test_config();
        let config_state = ConfigurationState::from_config(config.clone());
        let db_state = DatabaseState::new_temporary();
        let router_state = RouterState::new(config_state.inner.read().unwrap().routing.clone());
        let services_state = ServicesState::new();
        let connections_state = ConnectionsState::new();
        let rpc_state = RpcState::new();
        let auth_state = AuthenticationState::new();

        // Verify all pieces exist and are accessible
        let _routing = &config_state.inner.read().unwrap().routing;
        let _node_db = &db_state.inner.read().unwrap().node;
        let _table = &router_state.routing_table.inner.read().unwrap();
        let _messaging = &services_state.messaging;
        let _internet = &connections_state.internet;
        let _counter = &rpc_state.send_count.read().unwrap();
        let _nonce = &auth_state.nonce_counter.read().unwrap();
    }

    /// Create N independent "nodes" and verify they don't share state.
    #[test]
    fn n_independent_nodes() {
        use libp2p::identity::Keypair;
        use libqaul::connections::ConnectionModule;

        let n = 5;
        let config = RoutingOptions::default();

        let mut routers: Vec<Arc<RouterState>> = Vec::new();
        let mut peer_ids = Vec::new();

        for _ in 0..n {
            let kp = Keypair::generate_ed25519();
            let pid = kp.public().to_peer_id();
            let router = Arc::new(RouterState::new(config.clone()));
            router.connections.add_local_user(pid, pid);
            routers.push(router);
            peer_ids.push(pid);
        }

        // Make each node know its successor as a neighbour
        for i in 0..n - 1 {
            routers[i]
                .neighbours
                .update_node(ConnectionModule::Lan, peer_ids[i + 1], 5000);
        }

        // Node 0 should know node 1 as a neighbour
        assert_eq!(
            routers[0].neighbours.is_neighbour(&peer_ids[1]),
            ConnectionModule::Lan
        );

        // Node 4 should NOT know node 0
        assert_eq!(
            routers[4].neighbours.is_neighbour(&peer_ids[0]),
            ConnectionModule::None
        );
    }
}
