// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Node Metrics
//!
//! In-memory metric families. Call sites record with one line:
//!
//! ```rust,ignore
//! state.metrics.message_received(ReceiveOutcome::Ok);
//! ```
//!
//! Each family owns its name, description, unit and label keys, and
//! keeps one series per combination of label values, created on first
//! use. Nothing is persisted: all values reset on restart.

pub mod rpc;

use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use libp2p::PeerId;

use crate::connections::ConnectionModule;
use qaul_proto::qaul_net_messaging::dtn_response::{Reason, ResponseType};
use qaul_proto::qaul_rpc::Modules;
use qaul_proto::qaul_rpc_metrics as proto;

/// Upper bounds of the neighbour RTT histogram, in microseconds.
const RTT_BOUNDS_US: &[u64] = &[
    250, 500, 1_000, 2_500, 5_000, 10_000, 25_000, 50_000, 100_000, 250_000, 500_000, 1_000_000,
    2_500_000, 5_000_000,
];

/// Upper bounds of the message size histograms, in bytes.
const SIZE_BOUNDS_BYTES: &[u64] = &[
    256, 1_024, 4_096, 16_384, 65_536, 262_144, 1_048_576, 4_194_304,
];

/// Maximum sender / receiver pairs tracked in the transit table.
/// Further pairs are summed into the overflow counters.
const TRANSIT_MAX_PAIRS: usize = 4096;

/// Outcome of processing a message addressed to a local user.
///
/// The error variants are the rejection reasons of
/// `MessagingProcess::verify_and_dispatch`.
#[derive(Clone, Copy, Debug)]
pub enum ReceiveOutcome {
    /// payload was verified and dispatched
    Ok,
    /// container had no envelope
    NoEnvelope,
    /// sender or receiver id could not be parsed
    InvalidId,
    /// no public key known for the sender
    UnknownSender,
    /// signature verification failed
    VerifyFailed,
    /// decryption failed
    DecryptFailed,
    /// envelope payload could not be decoded
    DecodeError,
}

impl ReceiveOutcome {
    const ALL: [ReceiveOutcome; 7] = [
        ReceiveOutcome::Ok,
        ReceiveOutcome::NoEnvelope,
        ReceiveOutcome::InvalidId,
        ReceiveOutcome::UnknownSender,
        ReceiveOutcome::VerifyFailed,
        ReceiveOutcome::DecryptFailed,
        ReceiveOutcome::DecodeError,
    ];

    fn label(self) -> &'static str {
        match self {
            ReceiveOutcome::Ok => "ok",
            ReceiveOutcome::NoEnvelope => "no_envelope",
            ReceiveOutcome::InvalidId => "invalid_id",
            ReceiveOutcome::UnknownSender => "unknown_sender",
            ReceiveOutcome::VerifyFailed => "verify_failed",
            ReceiveOutcome::DecryptFailed => "decrypt_failed",
            ReceiveOutcome::DecodeError => "decode_error",
        }
    }
}

/// DTN protocol version
#[derive(Clone, Copy, Debug)]
pub enum DtnVersion {
    V1,
    V2,
}

impl DtnVersion {
    fn label(self) -> &'static str {
        match self {
            DtnVersion::V1 => "v1",
            DtnVersion::V2 => "v2",
        }
    }
}

/// Name, description, UCUM unit and label keys of a metric family.
struct Descriptor<const N: usize> {
    name: &'static str,
    description: &'static str,
    unit: &'static str,
    label_keys: [&'static str; N],
}

impl<const N: usize> Descriptor<N> {
    fn metric(&self, kind: proto::MetricKind, points: Vec<proto::DataPoint>) -> proto::Metric {
        proto::Metric {
            name: self.name.into(),
            description: self.description.into(),
            unit: self.unit.into(),
            kind: kind as i32,
            points,
        }
    }

    /// Label values are exported lower-case, so protobuf enum names
    /// (`as_str_name()`) can be used as label values directly.
    fn point(&self, labels: &[&str; N], value: proto::data_point::Value) -> proto::DataPoint {
        proto::DataPoint {
            attributes: self
                .label_keys
                .iter()
                .zip(labels)
                .map(|(key, value)| proto::Attribute {
                    key: key.to_string(),
                    value: value.to_lowercase(),
                })
                .collect(),
            value: Some(value),
        }
    }
}

/// Monotonic counters, one per combination of label values.
struct CounterVec<const N: usize> {
    descriptor: Descriptor<N>,
    series: Mutex<BTreeMap<[&'static str; N], u64>>,
}

impl<const N: usize> CounterVec<N> {
    fn new(descriptor: Descriptor<N>) -> Self {
        Self {
            descriptor,
            series: Mutex::new(BTreeMap::new()),
        }
    }

    /// Report these series with value 0 until they are first incremented.
    fn with_zeros(mut self, labels: impl IntoIterator<Item = [&'static str; N]>) -> Self {
        let series = self.series.get_mut().unwrap();
        for label in labels {
            series.insert(label, 0);
        }
        self
    }

    fn inc(&self, labels: [&'static str; N]) {
        *self.series.lock().unwrap().entry(labels).or_default() += 1;
    }

    fn to_proto(&self) -> proto::Metric {
        let series = self.series.lock().unwrap();
        let points = series
            .iter()
            .map(|(labels, value)| {
                self.descriptor
                    .point(labels, proto::data_point::Value::Counter(*value))
            })
            .collect();
        self.descriptor.metric(proto::MetricKind::Counter, points)
    }
}

/// Explicit-bucket histograms, one per combination of label values.
struct HistogramVec<const N: usize> {
    descriptor: Descriptor<N>,
    bounds: &'static [u64],
    series: Mutex<BTreeMap<[&'static str; N], Histogram>>,
}

impl<const N: usize> HistogramVec<N> {
    fn new(descriptor: Descriptor<N>, bounds: &'static [u64]) -> Self {
        Self {
            descriptor,
            bounds,
            series: Mutex::new(BTreeMap::new()),
        }
    }

    fn observe(&self, labels: [&'static str; N], value: u64) {
        self.series
            .lock()
            .unwrap()
            .entry(labels)
            .or_insert_with(|| Histogram::new(self.bounds))
            .observe(value);
    }

    fn to_proto(&self) -> proto::Metric {
        let series = self.series.lock().unwrap();
        let points = series
            .iter()
            .map(|(labels, histogram)| {
                self.descriptor.point(
                    labels,
                    proto::data_point::Value::Histogram(histogram.to_proto()),
                )
            })
            .collect();
        self.descriptor.metric(proto::MetricKind::Histogram, points)
    }
}

/// Explicit-bucket histogram of integer values.
struct Histogram {
    bounds: &'static [u64],
    /// one more bucket than bounds, for values above the last bound
    buckets: Vec<u64>,
    sum: u64,
    count: u64,
}

impl Histogram {
    fn new(bounds: &'static [u64]) -> Self {
        Self {
            bounds,
            buckets: vec![0; bounds.len() + 1],
            sum: 0,
            count: 0,
        }
    }

    fn observe(&mut self, value: u64) {
        let index = self
            .bounds
            .iter()
            .position(|bound| value <= *bound)
            .unwrap_or(self.bounds.len());
        self.buckets[index] += 1;
        self.sum += value;
        self.count += 1;
    }

    fn to_proto(&self) -> proto::Histogram {
        proto::Histogram {
            bounds: self.bounds.iter().map(|b| *b as f64).collect(),
            bucket_counts: self.buckets.clone(),
            sum: self.sum as f64,
            count: self.count,
        }
    }
}

/// Message count and bytes of one sender / receiver pair.
#[derive(Default, Clone, Copy)]
pub struct PairStats {
    pub message_count: u64,
    pub bytes: u64,
}

impl PairStats {
    pub fn add(&mut self, bytes: u64) {
        self.message_count += 1;
        self.bytes += bytes;
    }
}

/// (sender id, receiver id)
pub type PeerPair = (Vec<u8>, Vec<u8>);

pub(crate) fn pair_stats_to_proto(
    ((sender_id, receiver_id), stats): (PeerPair, PairStats),
) -> proto::PeerPairStats {
    proto::PeerPairStats {
        sender_id,
        receiver_id,
        message_count: stats.message_count,
        bytes: stats.bytes,
    }
}

/// Messages forwarded by this node, by sender and receiver.
#[derive(Default)]
struct TransitTable {
    pairs: HashMap<PeerPair, PairStats>,
    overflow: PairStats,
}

impl TransitTable {
    fn add(&mut self, sender_id: &[u8], receiver_id: &[u8], bytes: u64) {
        // the sender id of a forwarded message is unverified: only track
        // valid peer ids, so junk ids cannot bloat the table
        if PeerId::from_bytes(sender_id).is_err() {
            return;
        }
        let key = (sender_id.to_vec(), receiver_id.to_vec());
        let stats = if self.pairs.len() < TRANSIT_MAX_PAIRS || self.pairs.contains_key(&key) {
            self.pairs.entry(key).or_default()
        } else {
            &mut self.overflow
        };
        stats.add(bytes);
    }
}

/// Instance-based metrics state, lives on `QaulState`.
pub struct MetricsState {
    enabled: AtomicBool,
    start_time_unix_nano: u64,
    received: CounterVec<1>,
    dtn_custody: CounterVec<3>,
    rpc_requests: CounterVec<1>,
    forwarded_size: HistogramVec<1>,
    dtn_stored_size: HistogramVec<1>,
    neighbour_rtt: HistogramVec<1>,
    transit: Mutex<TransitTable>,
}

impl MetricsState {
    /// Create a new, disabled metrics state.
    /// Enabled during libqaul initialization from the configuration.
    pub fn new() -> Self {
        Self {
            enabled: AtomicBool::new(false),
            start_time_unix_nano: now_unix_nano(),
            received: CounterVec::new(Descriptor {
                name: "qaul.messaging.received",
                description: "Messages addressed to a local user, by processing outcome",
                unit: "{message}",
                label_keys: ["outcome"],
            })
            .with_zeros(ReceiveOutcome::ALL.map(|outcome| [outcome.label()])),
            dtn_custody: CounterVec::new(Descriptor {
                name: "qaul.dtn.custody",
                description: "DTN custody requests answered by this node",
                unit: "{message}",
                label_keys: ["version", "response", "reason"],
            }),
            rpc_requests: CounterVec::new(Descriptor {
                name: "qaul.rpc.requests",
                description: "RPC requests received from clients, by module",
                unit: "{request}",
                label_keys: ["module"],
            }),
            forwarded_size: HistogramVec::new(
                Descriptor {
                    name: "qaul.messaging.forwarded.size",
                    description:
                        "Size of messages forwarded for other nodes, by incoming transport",
                    unit: "By",
                    label_keys: ["transport"],
                },
                SIZE_BOUNDS_BYTES,
            ),
            dtn_stored_size: HistogramVec::new(
                Descriptor {
                    name: "qaul.dtn.stored.size",
                    description: "Size of DTN messages taken into custody",
                    unit: "By",
                    label_keys: ["version"],
                },
                SIZE_BOUNDS_BYTES,
            ),
            neighbour_rtt: HistogramVec::new(
                Descriptor {
                    name: "qaul.neighbour.rtt",
                    description: "Ping round trip time to direct neighbours",
                    unit: "us",
                    label_keys: ["transport"],
                },
                RTT_BOUNDS_US,
            ),
            transit: Mutex::new(TransitTable::default()),
        }
    }

    /// Enable or disable recording. Values recorded so far are kept.
    pub fn set_enabled(&self, enabled: bool) {
        self.enabled.store(enabled, Ordering::Relaxed);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// A message addressed to a local user was processed.
    pub fn message_received(&self, outcome: ReceiveOutcome) {
        if self.is_enabled() {
            self.received.inc([outcome.label()]);
        }
    }

    /// A message not addressed to us was scheduled for forwarding.
    pub fn message_forwarded(
        &self,
        transport: ConnectionModule,
        sender_id: &[u8],
        receiver_id: &[u8],
        bytes: usize,
    ) {
        if self.is_enabled() {
            self.forwarded_size
                .observe([transport.as_str_name()], bytes as u64);
            self.transit
                .lock()
                .unwrap()
                .add(sender_id, receiver_id, bytes as u64);
        }
    }

    /// A DTN custody request was answered.
    pub fn dtn_custody(&self, version: DtnVersion, response_type: ResponseType, reason: Reason) {
        if self.is_enabled() {
            self.dtn_custody.inc([
                version.label(),
                response_type.as_str_name(),
                reason.as_str_name(),
            ]);
        }
    }

    /// A DTN message was taken into custody.
    pub fn dtn_stored(&self, version: DtnVersion, bytes: usize) {
        if self.is_enabled() {
            self.dtn_stored_size
                .observe([version.label()], bytes as u64);
        }
    }

    /// A ping round trip to a neighbour was measured.
    pub fn neighbour_rtt(&self, transport: ConnectionModule, rtt: Duration) {
        if self.is_enabled() {
            self.neighbour_rtt
                .observe([transport.as_str_name()], rtt.as_micros() as u64);
        }
    }

    /// An RPC request was received from a client.
    pub fn rpc_request(&self, module: Modules) {
        if self.is_enabled() {
            self.rpc_requests.inc([module.as_str_name()]);
        }
    }

    /// Messages forwarded by this node, by sender and receiver.
    pub fn transit_peers(&self) -> proto::TransitPeersResponse {
        let table = self.transit.lock().unwrap();
        proto::TransitPeersResponse {
            pairs: table
                .pairs
                .iter()
                .map(|(pair, stats)| pair_stats_to_proto((pair.clone(), *stats)))
                .collect(),
            overflow_message_count: table.overflow.message_count,
            overflow_bytes: table.overflow.bytes,
        }
    }

    /// Build the node-wide snapshot.
    ///
    /// Series appear once they have been recorded, except the
    /// `outcome` series of `qaul.messaging.received`, which always
    /// report.
    pub fn snapshot(&self) -> proto::SnapshotResponse {
        proto::SnapshotResponse {
            enabled: self.is_enabled(),
            start_time_unix_nano: self.start_time_unix_nano,
            time_unix_nano: now_unix_nano(),
            metrics: vec![
                self.received.to_proto(),
                self.dtn_custody.to_proto(),
                self.rpc_requests.to_proto(),
                self.forwarded_size.to_proto(),
                self.dtn_stored_size.to_proto(),
                self.neighbour_rtt.to_proto(),
            ],
        }
    }
}

impl Default for MetricsState {
    fn default() -> Self {
        Self::new()
    }
}

fn now_unix_nano() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn metric<'a>(snapshot: &'a proto::SnapshotResponse, name: &str) -> &'a proto::Metric {
        snapshot.metrics.iter().find(|m| m.name == name).unwrap()
    }

    fn attributes(point: &proto::DataPoint) -> Vec<(&str, &str)> {
        point
            .attributes
            .iter()
            .map(|a| (a.key.as_str(), a.value.as_str()))
            .collect()
    }

    fn enabled() -> MetricsState {
        let metrics = MetricsState::new();
        metrics.set_enabled(true);
        metrics
    }

    #[test]
    fn disabled_state_records_nothing() {
        let metrics = MetricsState::new();
        metrics.message_received(ReceiveOutcome::Ok);
        metrics.rpc_request(Modules::Chat);

        let snapshot = metrics.snapshot();
        let received = metric(&snapshot, "qaul.messaging.received");
        assert!(received
            .points
            .iter()
            .all(|p| p.value == Some(proto::data_point::Value::Counter(0))));
        assert!(metric(&snapshot, "qaul.rpc.requests").points.is_empty());
    }

    #[test]
    fn received_outcomes_report_zero_until_recorded() {
        let metrics = enabled();
        metrics.message_received(ReceiveOutcome::DecryptFailed);

        let snapshot = metrics.snapshot();
        let received = metric(&snapshot, "qaul.messaging.received");
        assert_eq!(received.points.len(), ReceiveOutcome::ALL.len());
        let decrypt_failed = received
            .points
            .iter()
            .find(|p| attributes(p) == [("outcome", "decrypt_failed")])
            .unwrap();
        assert_eq!(
            decrypt_failed.value,
            Some(proto::data_point::Value::Counter(1))
        );
    }

    #[test]
    fn enum_label_values_are_lower_case() {
        let metrics = enabled();
        metrics.dtn_custody(DtnVersion::V2, ResponseType::Rejected, Reason::UserQuota);
        metrics.neighbour_rtt(ConnectionModule::Lan, Duration::from_micros(700));

        let snapshot = metrics.snapshot();
        let custody = metric(&snapshot, "qaul.dtn.custody");
        assert_eq!(
            attributes(&custody.points[0]),
            [
                ("version", "v2"),
                ("response", "rejected"),
                ("reason", "user_quota")
            ]
        );
        let rtt = metric(&snapshot, "qaul.neighbour.rtt");
        assert_eq!(attributes(&rtt.points[0]), [("transport", "lan")]);
        let Some(proto::data_point::Value::Histogram(histogram)) = &rtt.points[0].value else {
            panic!("rtt is not a histogram");
        };
        assert_eq!(histogram.count, 1);
        assert_eq!(histogram.sum, 700.0);
        // 700us falls into the (500, 1000] bucket
        assert_eq!(histogram.bucket_counts[2], 1);
    }

    #[test]
    fn transit_ignores_invalid_sender_ids() {
        let metrics = enabled();
        let sender = PeerId::random().to_bytes();
        let receiver = PeerId::random().to_bytes();
        metrics.message_forwarded(ConnectionModule::Ble, &sender, &receiver, 100);
        metrics.message_forwarded(ConnectionModule::Ble, &sender, &receiver, 50);
        metrics.message_forwarded(ConnectionModule::Ble, b"junk", &receiver, 10);

        let transit = metrics.transit_peers();
        assert_eq!(transit.pairs.len(), 1);
        assert_eq!(transit.pairs[0].message_count, 2);
        assert_eq!(transit.pairs[0].bytes, 150);
        assert_eq!(transit.overflow_message_count, 0);
    }
}
