// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! Prometheus text exposition format for a libqaul metrics snapshot.
//!
//! Used by `qauld-ctl metrics snapshot --format prometheus`, e.g. for a
//! node_exporter textfile collector.

use std::fmt::Write;

use qaul_proto::qaul_rpc_metrics::{
    data_point::Value, Attribute, Metric, MetricKind, SnapshotResponse,
};

/// Render a metrics snapshot in the Prometheus text exposition format.
pub fn format(snapshot: &SnapshotResponse) -> String {
    let mut out = String::new();
    for metric in &snapshot.metrics {
        format_metric(&mut out, metric);
    }
    out
}

fn format_metric(out: &mut String, metric: &Metric) {
    let kind = MetricKind::try_from(metric.kind).unwrap_or(MetricKind::Unspecified);
    let mut name = sanitize(&metric.name);
    if let Some(suffix) = unit_suffix(&metric.unit) {
        name.push('_');
        name.push_str(suffix);
    }
    let type_name = match kind {
        MetricKind::Counter => {
            name.push_str("_total");
            "counter"
        }
        MetricKind::Histogram => "histogram",
        MetricKind::Unspecified => "untyped",
    };

    let _ = writeln!(out, "# HELP {} {}", name, escape_help(&metric.description));
    let _ = writeln!(out, "# TYPE {} {}", name, type_name);

    for point in &metric.points {
        match &point.value {
            Some(Value::Counter(v)) => {
                let _ = writeln!(out, "{}{} {}", name, labels(&point.attributes, None), v);
            }
            Some(Value::Histogram(h)) => {
                let mut cumulative = 0u64;
                for (i, count) in h.bucket_counts.iter().enumerate() {
                    cumulative += count;
                    let le = match h.bounds.get(i) {
                        Some(bound) => bound.to_string(),
                        None => "+Inf".to_string(),
                    };
                    let _ = writeln!(
                        out,
                        "{}_bucket{} {}",
                        name,
                        labels(&point.attributes, Some(&le)),
                        cumulative
                    );
                }
                let point_labels = labels(&point.attributes, None);
                let _ = writeln!(out, "{}_sum{} {}", name, point_labels, h.sum);
                let _ = writeln!(out, "{}_count{} {}", name, point_labels, h.count);
            }
            None => {}
        }
    }
}

/// Map a UCUM unit to the Prometheus base-unit name suffix.
/// Annotation units such as `{message}` carry no suffix.
fn unit_suffix(unit: &str) -> Option<&'static str> {
    match unit {
        "By" => Some("bytes"),
        "s" => Some("seconds"),
        "ms" => Some("milliseconds"),
        "us" => Some("microseconds"),
        _ => None,
    }
}

/// Replace every character not allowed in a Prometheus metric or
/// label name with `_`.
fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect()
}

fn labels(attributes: &[Attribute], le: Option<&str>) -> String {
    let mut pairs: Vec<String> = attributes
        .iter()
        .map(|a| format!("{}=\"{}\"", sanitize(&a.key), escape_label_value(&a.value)))
        .collect();
    if let Some(le) = le {
        pairs.push(format!("le=\"{}\"", le));
    }
    if pairs.is_empty() {
        String::new()
    } else {
        format!("{{{}}}", pairs.join(","))
    }
}

fn escape_help(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\n', "\\n")
}

fn escape_label_value(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use qaul_proto::qaul_rpc_metrics::{
        data_point::Value, Attribute, DataPoint, Histogram, Metric, MetricKind,
    };

    fn snapshot(metrics: Vec<Metric>) -> SnapshotResponse {
        SnapshotResponse {
            enabled: true,
            start_time_unix_nano: 0,
            time_unix_nano: 0,
            metrics,
        }
    }

    fn metric(name: &str, unit: &str, kind: MetricKind, points: Vec<DataPoint>) -> Metric {
        Metric {
            name: name.to_string(),
            description: "a description".to_string(),
            unit: unit.to_string(),
            kind: kind as i32,
            points,
        }
    }

    fn point(attributes: &[(&str, &str)], value: Value) -> DataPoint {
        DataPoint {
            attributes: attributes
                .iter()
                .map(|(k, v)| Attribute {
                    key: k.to_string(),
                    value: v.to_string(),
                })
                .collect(),
            value: Some(value),
        }
    }

    #[test]
    fn counter_gets_total_suffix_help_and_type() {
        let out = format(&snapshot(vec![metric(
            "qaul.messaging.received",
            "{message}",
            MetricKind::Counter,
            vec![
                point(&[("outcome", "ok")], Value::Counter(3)),
                point(&[("outcome", "decrypt_failed")], Value::Counter(1)),
            ],
        )]));

        assert_eq!(
            out,
            "# HELP qaul_messaging_received_total a description\n\
             # TYPE qaul_messaging_received_total counter\n\
             qaul_messaging_received_total{outcome=\"ok\"} 3\n\
             qaul_messaging_received_total{outcome=\"decrypt_failed\"} 1\n"
        );
    }

    #[test]
    fn point_without_attributes_has_no_braces() {
        let out = format(&snapshot(vec![metric(
            "qaul.rpc.requests",
            "{request}",
            MetricKind::Counter,
            vec![point(&[], Value::Counter(5))],
        )]));

        assert!(out.contains("\nqaul_rpc_requests_total 5\n"), "{out}");
    }

    #[test]
    fn histogram_buckets_are_cumulative_with_inf_sum_and_count() {
        let out = format(&snapshot(vec![metric(
            "qaul.messaging.forwarded.size",
            "By",
            MetricKind::Histogram,
            vec![point(
                &[("transport", "lan")],
                Value::Histogram(Histogram {
                    bounds: vec![100.0, 1000.0],
                    bucket_counts: vec![1, 2, 3],
                    sum: 5150.5,
                    count: 6,
                }),
            )],
        )]));

        assert_eq!(
            out,
            "# HELP qaul_messaging_forwarded_size_bytes a description\n\
             # TYPE qaul_messaging_forwarded_size_bytes histogram\n\
             qaul_messaging_forwarded_size_bytes_bucket{transport=\"lan\",le=\"100\"} 1\n\
             qaul_messaging_forwarded_size_bytes_bucket{transport=\"lan\",le=\"1000\"} 3\n\
             qaul_messaging_forwarded_size_bytes_bucket{transport=\"lan\",le=\"+Inf\"} 6\n\
             qaul_messaging_forwarded_size_bytes_sum{transport=\"lan\"} 5150.5\n\
             qaul_messaging_forwarded_size_bytes_count{transport=\"lan\"} 6\n"
        );
    }

    #[test]
    fn histogram_without_attributes_only_has_le_label() {
        let out = format(&snapshot(vec![metric(
            "qaul.neighbour.rtt",
            "us",
            MetricKind::Histogram,
            vec![point(
                &[],
                Value::Histogram(Histogram {
                    bounds: vec![1000.0],
                    bucket_counts: vec![0, 1],
                    sum: 2000.0,
                    count: 1,
                }),
            )],
        )]));

        assert!(
            out.contains("\nqaul_neighbour_rtt_microseconds_bucket{le=\"1000\"} 0\n"),
            "{out}"
        );
        assert!(
            out.contains("\nqaul_neighbour_rtt_microseconds_sum 2000\n"),
            "{out}"
        );
    }

    #[test]
    fn label_values_and_help_are_escaped() {
        let mut m = metric(
            "qaul.test",
            "",
            MetricKind::Counter,
            vec![point(&[("peer", "a\"b\\c\nd")], Value::Counter(1))],
        );
        m.description = "line\\one\nline two".to_string();
        let out = format(&snapshot(vec![m]));

        assert_eq!(
            out,
            "# HELP qaul_test_total line\\\\one\\nline two\n\
             # TYPE qaul_test_total counter\n\
             qaul_test_total{peer=\"a\\\"b\\\\c\\nd\"} 1\n"
        );
    }

    #[test]
    fn invalid_name_characters_are_replaced() {
        let out = format(&snapshot(vec![metric(
            "qaul.dtn-custody",
            "",
            MetricKind::Counter,
            vec![point(&[("dtn.version", "v2")], Value::Counter(1))],
        )]));

        assert!(
            out.contains("\nqaul_dtn_custody_total{dtn_version=\"v2\"} 1\n"),
            "{out}"
        );
    }

    #[test]
    fn multiple_metrics_are_rendered_in_order() {
        let out = format(&snapshot(vec![
            metric(
                "qaul.a",
                "",
                MetricKind::Counter,
                vec![point(&[], Value::Counter(1))],
            ),
            metric(
                "qaul.b",
                "",
                MetricKind::Counter,
                vec![point(&[], Value::Counter(2))],
            ),
        ]));

        let a = out.find("qaul_a_total 1").expect("qaul_a missing");
        let b = out.find("qaul_b_total 2").expect("qaul_b missing");
        assert!(a < b);
    }

    #[test]
    fn empty_snapshot_renders_nothing() {
        assert_eq!(format(&snapshot(vec![])), "");
    }
}
