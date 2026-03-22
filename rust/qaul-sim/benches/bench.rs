//! Criterion benchmarks for qaul routing simulation.

use criterion::{criterion_group, criterion_main, Criterion};
use libqaul::storage::configuration::RoutingOptions;
use qaul_sim::simulator::Simulator;
use qaul_sim::topology::Topology;

fn default_config() -> RoutingOptions {
    RoutingOptions {
        sending_table_period: 10,
        ping_neighbour_period: 5,
        hop_count_penalty: 10,
        maintain_period_limit: 300,
    }
}

fn bench_line_convergence(c: &mut Criterion) {
    c.bench_function("line_10_convergence", |b| {
        b.iter(|| {
            let topo = Topology::line(10, 5000);
            let mut sim = Simulator::new(topo, default_config());
            let mut rng = rand::rng();
            sim.ticks_to_convergence(50, &mut rng)
        })
    });
}

fn bench_ring_convergence(c: &mut Criterion) {
    c.bench_function("ring_10_convergence", |b| {
        b.iter(|| {
            let topo = Topology::ring(10, 5000);
            let mut sim = Simulator::new(topo, default_config());
            let mut rng = rand::rng();
            sim.ticks_to_convergence(50, &mut rng)
        })
    });
}

fn bench_grid_convergence(c: &mut Criterion) {
    c.bench_function("grid_4x4_convergence", |b| {
        b.iter(|| {
            let topo = Topology::grid(4, 4, 5000);
            let mut sim = Simulator::new(topo, default_config());
            let mut rng = rand::rng();
            sim.ticks_to_convergence(50, &mut rng)
        })
    });
}

fn bench_full_mesh_convergence(c: &mut Criterion) {
    c.bench_function("full_mesh_8_convergence", |b| {
        b.iter(|| {
            let topo = Topology::full_mesh(8, 5000);
            let mut sim = Simulator::new(topo, default_config());
            let mut rng = rand::rng();
            sim.ticks_to_convergence(20, &mut rng)
        })
    });
}

fn bench_single_tick(c: &mut Criterion) {
    c.bench_function("single_tick_10_node_line", |b| {
        let topo = Topology::line(10, 5000);
        let mut sim = Simulator::new(topo, default_config());
        let mut rng = rand::rng();
        // Pre-converge
        sim.run(20, &mut rng);
        b.iter(|| {
            sim.tick(&mut rng);
        })
    });
}

fn bench_ble_mesh_convergence(c: &mut Criterion) {
    c.bench_function("ble_mesh_8_convergence", |b| {
        b.iter(|| {
            let mut sim = qaul_sim::scenarios::ble_only_mesh(8);
            let mut rng = rand::rng();
            sim.ticks_to_convergence(40, &mut rng)
        })
    });
}

fn bench_mixed_ble_lan_convergence(c: &mut Criterion) {
    c.bench_function("mixed_ble_lan_10_convergence", |b| {
        b.iter(|| {
            let mut sim = qaul_sim::scenarios::mixed_ble_lan(10);
            let mut rng = rand::rng();
            sim.ticks_to_convergence(50, &mut rng)
        })
    });
}

fn bench_internet_relay_convergence(c: &mut Criterion) {
    c.bench_function("internet_relay_8_convergence", |b| {
        b.iter(|| {
            let mut sim = qaul_sim::scenarios::internet_relay(8);
            let mut rng = rand::rng();
            sim.ticks_to_convergence(20, &mut rng)
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
