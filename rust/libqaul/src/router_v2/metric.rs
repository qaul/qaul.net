//! All metrics related functionality, like the bucketed weights and all.

use crate::connections::ConnectionModule;

/// returns the weighst for each trasnport
pub fn transport_weight(transport: ConnectionModule) -> u16 {
    match transport {
        ConnectionModule::Ble1m => 50,
        ConnectionModule::BleCoded => 70,
        ConnectionModule::Lan => 10,
        ConnectionModule::Local => 0,
        ConnectionModule::Internet => 15,
        // so that it will get calculated as the lowest
        ConnectionModule::None => u16::MAX,
    }
}

/// calclates the penalty for a BLE transport using the RSSI
pub fn rssi_penalty(t: ConnectionModule, rssi_dbm: Option<i8>) -> u16 {
    if !matches!(t, ConnectionModule::Ble1m | ConnectionModule::BleCoded) {
        return 0;
    }
    let Some(rssi) = rssi_dbm else { return 0 };
    if rssi >= -60 {
        0
    } else if rssi >= -75 {
        5
    } else if rssi >= -85 {
        10
    } else {
        20
    }
}

/// the cost to send a message to the next hop
pub fn hop_cost(transport: ConnectionModule, rssi_dbm: Option<i8>) -> u16 {
    transport_weight(transport).saturating_add(rssi_penalty(transport, rssi_dbm))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---------- transport_weight ----------

    #[test]
    fn transport_weight_matches_spec_for_every_variant() {
        assert_eq!(transport_weight(ConnectionModule::Lan), 10);
        assert_eq!(transport_weight(ConnectionModule::Internet), 15);
        assert_eq!(transport_weight(ConnectionModule::Ble1m), 50);
        assert_eq!(transport_weight(ConnectionModule::BleCoded), 70);
        assert_eq!(transport_weight(ConnectionModule::Local), 0);
    }

    /// `None` is a non-transport — it must rank as the worst possible
    /// path so that `min_by_key(metric)` never picks it if a stray
    /// `None` ever appears in a routing entry.
    #[test]
    fn transport_weight_for_none_is_max() {
        assert_eq!(transport_weight(ConnectionModule::None), u16::MAX);
    }

    // ---------- rssi_penalty (non-BLE shortcuts) ----------

    /// RSSI is a BLE concept; on wired/internet transports the value
    /// must be ignored even when supplied.
    #[test]
    fn rssi_penalty_zero_for_non_ble_with_any_rssi() {
        for t in [
            ConnectionModule::Lan,
            ConnectionModule::Internet,
            ConnectionModule::Local,
            ConnectionModule::None,
        ] {
            assert_eq!(rssi_penalty(t, Some(-90)), 0);
            assert_eq!(rssi_penalty(t, Some(-30)), 0);
            assert_eq!(rssi_penalty(t, None), 0);
        }
    }

    /// BLE link without an RSSI measurement gets no penalty — the spec
    /// doesn't punish unknown signal strength.
    #[test]
    fn rssi_penalty_zero_for_ble_with_no_rssi() {
        assert_eq!(rssi_penalty(ConnectionModule::Ble1m, None), 0);
        assert_eq!(rssi_penalty(ConnectionModule::BleCoded, None), 0);
    }

    // ---------- rssi_penalty (bucket boundaries) ----------

    /// At each boundary the inclusive side belongs to the *better* bucket
    /// — pins the comparison operators against accidental drift.
    #[test]
    fn rssi_penalty_bucket_boundaries_ble1m() {
        let t = ConnectionModule::Ble1m;
        // Bucket 1: rssi >= -60 → 0
        assert_eq!(rssi_penalty(t, Some(-60)), 0);
        assert_eq!(rssi_penalty(t, Some(-30)), 0);
        // Bucket 2: -75 <= rssi <= -61 → 5
        assert_eq!(rssi_penalty(t, Some(-61)), 5);
        assert_eq!(rssi_penalty(t, Some(-75)), 5);
        // Bucket 3: -85 <= rssi <= -76 → 10
        assert_eq!(rssi_penalty(t, Some(-76)), 10);
        assert_eq!(rssi_penalty(t, Some(-85)), 10);
        // Bucket 4: rssi < -85 → 20
        assert_eq!(rssi_penalty(t, Some(-86)), 20);
        assert_eq!(rssi_penalty(t, Some(-128)), 20);
    }

    /// BleCoded uses the same RSSI buckets as Ble1m — the penalty
    /// function doesn't differentiate between the two PHYs.
    #[test]
    fn rssi_penalty_bucket_boundaries_ble_coded() {
        let t = ConnectionModule::BleCoded;
        assert_eq!(rssi_penalty(t, Some(-60)), 0);
        assert_eq!(rssi_penalty(t, Some(-61)), 5);
        assert_eq!(rssi_penalty(t, Some(-75)), 5);
        assert_eq!(rssi_penalty(t, Some(-76)), 10);
        assert_eq!(rssi_penalty(t, Some(-85)), 10);
        assert_eq!(rssi_penalty(t, Some(-86)), 20);
    }

    // ---------- hop_cost (composition) ----------

    #[test]
    fn hop_cost_lan_ignores_rssi() {
        // 10 + 0 = 10, even with a value that would penalise BLE.
        assert_eq!(hop_cost(ConnectionModule::Lan, Some(-90)), 10);
        assert_eq!(hop_cost(ConnectionModule::Lan, None), 10);
    }

    #[test]
    fn hop_cost_internet_with_no_rssi_is_just_weight() {
        assert_eq!(hop_cost(ConnectionModule::Internet, None), 15);
    }

    #[test]
    fn hop_cost_ble1m_with_weak_rssi_sums_weight_and_penalty() {
        // 50 + 10 (rssi=-80 → bucket 3) = 60. Pins the plan's worked example.
        assert_eq!(hop_cost(ConnectionModule::Ble1m, Some(-80)), 60);
    }

    #[test]
    fn hop_cost_ble_coded_with_strong_rssi_has_no_penalty() {
        // 70 + 0 = 70
        assert_eq!(hop_cost(ConnectionModule::BleCoded, Some(-50)), 70);
    }

    #[test]
    fn hop_cost_ble_coded_at_max_penalty() {
        // 70 + 20 = 90
        assert_eq!(hop_cost(ConnectionModule::BleCoded, Some(-100)), 90);
    }

    /// `None` weight is `u16::MAX`; adding any penalty must saturate
    /// rather than wrap around. This exists to guard against future
    /// changes that might switch to plain `+`.
    #[test]
    fn hop_cost_saturates_at_u16_max_for_none_transport() {
        assert_eq!(hop_cost(ConnectionModule::None, None), u16::MAX);
        // Even though None is non-BLE so the penalty is 0 today, this
        // pins the saturating shape against future weight changes.
        assert_eq!(hop_cost(ConnectionModule::None, Some(-90)), u16::MAX);
    }
}
