//! Criterion benchmarks for qaul routing simulation.
//!
//! Each `SimNode` wraps a real `libqaul::services::ServicesState`, which
//! opens three temporary sled databases per node (Feed, Messaging, DTN).
//! Those opens and (especially) their cleanup-on-drop would dwarf the
//! routing work we want to measure, so we use `iter_custom` to put
//! `Simulator::new` and `drop(sim)` outside the timed region and only
//! time the `ticks_to_convergence` / `tick` call itself.

use criterion::{criterion_group, criterion_main, Criterion};
use libqaul::storage::configuration::RoutingOptions;
use qaul_sim::simulator::Simulator;
use qaul_sim::topology::Topology;
use std::hint::black_box;
use std::time::{Duration, Instant};

fn default_config() -> RoutingOptions {
    RoutingOptions {
        sending_table_period: 10,
        ping_neighbour_period: 5,
        hop_count_penalty: 10,
        maintain_period_limit: 300,
    }
}

/// Helper: run the convergence routine `iters` times, timing only the
/// `ticks_to_convergence` call — not simulator construction or drop.
fn timed_convergence<F>(iters: u64, max_ticks: u64, build_sim: F) -> Duration
where
    F: Fn() -> Simulator,
{
    let mut total = Duration::ZERO;
    for _ in 0..iters {
        let mut sim = build_sim();
        let mut rng = rand::rng();
        let start = Instant::now();
        black_box(sim.ticks_to_convergence(max_ticks, &mut rng));
        total += start.elapsed();
        drop(sim);
    }
    total
}

fn bench_line_convergence(c: &mut Criterion) {
    c.bench_function("line_10_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 50, || {
                Simulator::new(Topology::line(10, 5000), default_config())
            })
        })
    });
}

fn bench_ring_convergence(c: &mut Criterion) {
    c.bench_function("ring_10_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 50, || {
                Simulator::new(Topology::ring(10, 5000), default_config())
            })
        })
    });
}

fn bench_grid_convergence(c: &mut Criterion) {
    c.bench_function("grid_4x4_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 50, || {
                Simulator::new(Topology::grid(4, 4, 5000), default_config())
            })
        })
    });
}

fn bench_full_mesh_convergence(c: &mut Criterion) {
    c.bench_function("full_mesh_8_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 20, || {
                Simulator::new(Topology::full_mesh(8, 5000), default_config())
            })
        })
    });
}

fn bench_single_tick(c: &mut Criterion) {
    c.bench_function("single_tick_10_node_line", |b| {
        let topo = Topology::line(10, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();
        sim.run(20, &mut rng);
        b.iter(|| {
            sim.tick(&mut rng);
        })
    });
}

fn bench_ble_mesh_convergence(c: &mut Criterion) {
    c.bench_function("ble_mesh_8_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 40, || qaul_sim::scenarios::ble_only_mesh(8))
        })
    });
}

fn bench_mixed_ble_lan_convergence(c: &mut Criterion) {
    c.bench_function("mixed_ble_lan_10_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 50, || qaul_sim::scenarios::mixed_ble_lan(10))
        })
    });
}

fn bench_internet_relay_convergence(c: &mut Criterion) {
    c.bench_function("internet_relay_8_convergence", |b| {
        b.iter_custom(|iters| {
            timed_convergence(iters, 20, || qaul_sim::scenarios::internet_relay(8))
        })
    });
}

criterion_group!(
    benches,
    bench_line_convergence,
    bench_ring_convergence,
    bench_grid_convergence,
    bench_full_mesh_convergence,
    bench_single_tick,
    bench_ble_mesh_convergence,
    bench_mixed_ble_lan_convergence,
    bench_internet_relay_convergence,
);
criterion_main!(benches);
