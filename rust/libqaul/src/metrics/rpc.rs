// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Metrics Messages
//!
//! `Snapshot` and `SetEnabled` are node-wide and open like the Debug
//! module. The per-peer and per-user custom metrics reveal who talks
//! to whom, so they require an authenticated session of the caller.

use crate::rpc::authentication::Authentication;
use crate::rpc::Rpc;
use crate::storage::configuration::Configuration;
use crate::storage::database::DataBase;

use super::pair_stats_to_proto;

/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_metrics as proto;

use proto::{
    DtnStorageRequest, DtnStorageResponse, MetricsService, SetEnabledRequest, SnapshotRequest,
    SnapshotResponse, TransitPeersRequest, TransitPeersResponse, UserStorageRequest,
    UserStorageResponse,
};
use qaul_proto::qaul_common::{Ack, RpcError};

/// RPC Metrics Module
pub struct Metrics {}

impl Metrics {
    /// Process incoming RPC request messages for the metrics module
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, user_id: Vec<u8>, request_id: String) {
        let ctx = crate::RequestContext {
            state,
            user_id,
            request_id: request_id.clone(),
        };
        let response_bytes = proto::dispatch::<crate::RequestContext, Metrics>(&ctx, data);
        Rpc::send_message(
            state,
            response_bytes,
            crate::rpc::proto::Modules::Metrics.into(),
            request_id,
            Vec::new(),
        );
    }
}

impl MetricsService<crate::RequestContext<'_>> for Metrics {
    fn snapshot(
        ctx: &crate::RequestContext<'_>,
        _req: SnapshotRequest,
    ) -> Result<SnapshotResponse, RpcError> {
        Ok(ctx.state.metrics.snapshot())
    }

    fn set_enabled(
        ctx: &crate::RequestContext<'_>,
        req: SetEnabledRequest,
    ) -> Result<Ack, RpcError> {
        ctx.state.metrics.set_enabled(req.enabled);
        Configuration::set_metrics_enabled(ctx.state, req.enabled);
        log::info!("metrics enabled: {}", req.enabled);
        Ok(Ack {})
    }

    fn dtn_storage(
        ctx: &crate::RequestContext<'_>,
        _req: DtnStorageRequest,
    ) -> Result<DtnStorageResponse, RpcError> {
        Authentication::require_session(ctx)?;
        let dtn = &ctx.state.services.dtn;
        let (v1_bytes, v1_message_count) = dtn.get_state();
        Ok(DtnStorageResponse {
            v1_message_count,
            v1_bytes,
            v2: dtn
                .v2_storage_by_pair()
                .into_iter()
                .map(pair_stats_to_proto)
                .collect(),
        })
    }

    fn transit_peers(
        ctx: &crate::RequestContext<'_>,
        _req: TransitPeersRequest,
    ) -> Result<TransitPeersResponse, RpcError> {
        Authentication::require_session(ctx)?;
        Ok(ctx.state.metrics.transit_peers())
    }

    fn user_storage(
        ctx: &crate::RequestContext<'_>,
        _req: UserStorageRequest,
    ) -> Result<UserStorageResponse, RpcError> {
        let user_id = Authentication::require_session(ctx)?;
        let bytes = DataBase::get_user_db(ctx.state, user_id)
            .size_on_disk()
            .map_err(|e| RpcError {
                code: 2,
                message: format!("failed to read database size: {}", e),
                details: String::new(),
            })?;
        Ok(UserStorageResponse {
            user_id: user_id.to_bytes(),
            bytes,
        })
    }
}
