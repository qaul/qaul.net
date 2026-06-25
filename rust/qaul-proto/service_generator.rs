// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

use prost_build::{Service, ServiceGenerator};

pub struct QaulServiceGenerator;

impl ServiceGenerator for QaulServiceGenerator {
    fn generate(&mut self, service: Service, buf: &mut String) {
        buf.push_str(&build_service_code(&service));
    }
}

/// Strips the "Service" suffix to get the envelope message name.
/// "DebugService" -> "Debug"
fn envelope_name(service_name: &str) -> &str {
    service_name
        .strip_suffix("Service")
        .unwrap_or(service_name)
}

/// Converts PascalCase to snake_case.
/// "Debug" -> "debug", "StoragePath" -> "storage_path"
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_lowercase().next().unwrap());
    }
    result
}

/// Maps a Rust type path to the oneof variant name.
/// prost emits variant names from the oneof field name (snake_case -> PascalCase).
/// Local types: the variant name equals the struct name (e.g. HeartbeatRequest -> HeartbeatRequest).
/// Cross-package Ack: variant name is "Ack".
/// Cross-package RpcError: variant name is "Error" (because the oneof field is named `error`).
fn oneof_variant(rust_type: &str) -> &str {
    let leaf = rust_type.rsplit("::").next().unwrap_or(rust_type);
    match leaf {
        "RpcError" => "Error",
        other => other,
    }
}

fn build_service_code(service: &Service) -> String {
    let env = envelope_name(&service.name);
    let env_mod = to_snake_case(env);
    // Path from within the generated debug module to common types:
    // qaul.rpc.debug -> super(debug) -> super(rpc) -> common::*
    let common_err = "crate::qaul_common::RpcError";

    let mut buf = String::new();

    // ---- trait definition ----
    buf.push_str(&format!("pub trait {}<S> {{\n", service.name));
    for method in &service.methods {
        buf.push_str(&format!(
            "    fn {}(state: &S, req: {}) -> Result<{}, {}>;\n",
            method.name, method.input_type, method.output_type, common_err
        ));
    }
    buf.push_str("}\n\n");

    // ---- dispatch function ----
    buf.push_str(&format!(
        "pub fn dispatch<S, T: {}<S>>(state: &S, data: Vec<u8>) -> Vec<u8> {{\n",
        service.name
    ));
    buf.push_str("    use prost::Message;\n");
    buf.push_str(&format!(
        "    let response_oneof = match {}::decode(&data[..]) {{\n",
        env
    ));
    buf.push_str("        Ok(envelope) => match envelope.message {\n");

    for method in &service.methods {
        let req_variant = oneof_variant(&method.input_type);
        let resp_variant = oneof_variant(&method.output_type);
        buf.push_str(&format!(
            "            Some({}::Message::{}(req)) => match T::{}(state, req) {{\n",
            env_mod, req_variant, method.name
        ));
        buf.push_str(&format!(
            "                Ok(resp) => {}::Message::{}(resp),\n",
            env_mod, resp_variant
        ));
        buf.push_str(&format!(
            "                Err(e) => {}::Message::Error(e),\n",
            env_mod
        ));
        buf.push_str("            },\n");
    }

    buf.push_str(&format!(
        "            _ => {}::Message::Error({} {{\n",
        env_mod, common_err
    ));
    buf.push_str("                code: 1,\n");
    buf.push_str(&format!(
        "                message: \"unexpected or unset {} oneof variant\".into(),\n",
        env
    ));
    buf.push_str("                details: String::new(),\n");
    buf.push_str("            }),\n");
    buf.push_str("        },\n"); // close inner match
    buf.push_str(&format!(
        "        Err(e) => {}::Message::Error({} {{\n",
        env_mod, common_err
    ));
    buf.push_str("            code: 1,\n");
    buf.push_str("            message: format!(\"decode failure: {}\", e),\n");
    buf.push_str("            details: String::new(),\n");
    buf.push_str("        }),\n");
    buf.push_str("    };\n"); // close outer match
    buf.push_str(&format!(
        "    let envelope = {} {{ message: Some(response_oneof) }};\n",
        env
    ));
    buf.push_str("    let mut out = Vec::with_capacity(envelope.encoded_len());\n");
    buf.push_str("    envelope.encode(&mut out).expect(\"Vec<u8> provides capacity as needed\");\n");
    buf.push_str("    out\n");
    buf.push_str("}\n\n");

    buf
}
