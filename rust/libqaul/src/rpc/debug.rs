// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # RPC Debug Messages
//!
//! Messages to debug libqaul

use super::Rpc;
use crate::storage::configuration::Configuration;
use crate::storage::Storage;

/// Import protobuf message definition
pub use qaul_proto::qaul_rpc_debug as proto;

use proto::{
    DebugService, DeleteLibqaulLogsRequest, HeartbeatRequest, HeartbeatResponse, LogToFile, Panic,
    StoragePathRequest, StoragePathResponse,
};
use qaul_proto::qaul_common::{Ack, RpcError};

/// RPC Debugging Module
pub struct Debug {}

impl DebugService<crate::QaulState> for Debug {
    fn heartbeat(_state: &crate::QaulState, _req: HeartbeatRequest) -> Result<HeartbeatResponse, RpcError> {
        Ok(HeartbeatResponse {})
    }

    fn trigger_panic(_state: &crate::QaulState, _req: Panic) -> Result<Ack, RpcError> {
        log::error!("Libqaul will panic");
        panic!("Libqaul panics for debugging reasons");
    }

    fn set_log_to_file(state: &crate::QaulState, req: LogToFile) -> Result<Ack, RpcError> {
        if req.enable {
            state.filelogger.enable(true);
            if !Configuration::get_debug_log(state) {
                Configuration::enable_debug_log(state, true);
                Configuration::save(state);
                log::info!("starting debug log..");
            }
        } else {
            if Configuration::get_debug_log(state) {
                Configuration::enable_debug_log(state, false);
                Configuration::save(state);
                log::info!("stop debug log..");
            }
            state.filelogger.enable(false);
        }
        Ok(Ack {})
    }

    fn storage_path(state: &crate::QaulState, _req: StoragePathRequest) -> Result<StoragePathResponse, RpcError> {
        Ok(StoragePathResponse {
            storage_path: Storage::get_path(state),
        })
    }

    fn delete_libqaul_logs(_state: &crate::QaulState, _req: DeleteLibqaulLogsRequest) -> Result<Ack, RpcError> {
        // TODO: implement log deletion
        Err(RpcError {
            code: 2,
            message: "delete_libqaul_logs not yet implemented".into(),
            details: String::new(),
        })
    }
}

impl Debug {
    /// Process incoming RPC request messages for debug module
    pub fn rpc(state: &crate::QaulState, data: Vec<u8>, _user_id: Vec<u8>, request_id: String) {
        let response_bytes = proto::dispatch::<crate::QaulState, Debug>(state, data);
        Rpc::send_message(
            state,
            response_bytes,
            crate::rpc::proto::Modules::Debug.into(),
            request_id,
            Vec::new(),
        );
    }
}
