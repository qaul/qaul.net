//! Network simulation utilities
//!
//! Provides latency sampling with jitter and packet loss modeling.

use rand::Rng;

use crate::topology::Link;

/// Sample an RTT from a link, applying jitter via uniform distribution.
/// Returns `None` if the packet is "lost" due to the link's loss probability.
pub fn sample_rtt(link: &Link, rng: &mut impl Rng) -> Option<u32> {
    // packet loss check
    if link.loss > 0.0 && rng.random::<f64>() < link.loss {
        return None;
    }

    if link.jitter_us == 0 {
        return Some(link.base_rtt_us);
    }

    // uniform jitter: [base - jitter, base + jitter]
    let lo = link.base_rtt_us.saturating_sub(link.jitter_us);
    let hi = link.base_rtt_us.saturating_add(link.jitter_us);
    Some(rng.random_range(lo..=hi))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topology::Link;

    #[test]
    fn no_jitter_returns_base() {
        let link = Link::new(5000);
        let mut rng = rand::rng();
        for _ in 0..100 {
            assert_eq!(sample_rtt(&link, &mut rng), Some(5000));
        }
    }

    #[test]
    fn jitter_stays_in_range() {
        let link = Link::new(5000).with_jitter(1000);
        let mut rng = rand::rng();
        for _ in 0..1000 {
            if let Some(rtt) = sample_rtt(&link, &mut rng) {
                assert!(rtt >= 4000 && rtt <= 6000, "rtt={} out of range", rtt);
            }
        }
    }

    #[test]
    fn total_loss() {
        let link = Link::new(5000).with_loss(1.0);
        let mut rng = rand::rng();
        for _ in 0..100 {
            assert_eq!(sample_rtt(&link, &mut rng), None);
        }
    }

    #[test]
    fn no_loss() {
        let link = Link::new(5000).with_loss(0.0);
        let mut rng = rand::rng();
        for _ in 0..100 {
            assert!(sample_rtt(&link, &mut rng).is_some());
        }
    }
}
