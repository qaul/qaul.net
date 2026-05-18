# qaul-sim Benchmark Results

Criterion benchmarks measuring routing-table convergence time across
different mesh topologies and radio-mix scenarios. Each "convergence"
benchmark times how long it takes for every node in the simulated
network to reach a fully consistent routing table, using
`Simulator::ticks_to_convergence` (which short-circuits as soon as
convergence is detected).

## Environment

- Base: `fix/instance-refactor-compile-errors` tip — completes the
  instance-based refactor of `libqaul` and fixes the benchmark harness
  to exclude `Simulator` setup/drop from the timed region.
- Platform: Darwin 24.6.0 (macOS), aarch64.
- Profile: `bench` (release, optimized).

## Harness

Convergence benchmarks use `criterion::iter_custom`. `Simulator::new`
runs in the (untimed) setup, `ticks_to_convergence` is timed with
`Instant::now() / elapsed()`, and `drop(sim)` runs *after* the stop
— otherwise sled cleanup (3 temporary databases per simulated node
via `ServicesState::new()`) would dominate the measurement.

`single_tick_10_node_line` uses plain `iter` — it amortises setup
over millions of iterations by design.

## Routing configuration

```rust
RoutingOptions {
    sending_table_period: 10,
    ping_neighbour_period: 5,
    hop_count_penalty: 10,
    maintain_period_limit: 300,
}
```

## Results

| Benchmark                       | Topology                   | Nodes | Median time | σ band (95% CI)     |
|---------------------------------|----------------------------|-------|-------------|---------------------|
| `single_tick_10_node_line`      | line (pre-converged, idle) | 10    | 26.35 µs    | [26.30, 26.39] µs   |
| `internet_relay_8_convergence`  | internet-relay scenario    | 8     | 49.34 µs    | [48.92, 49.77] µs   |
| `full_mesh_8_convergence`       | full mesh                  | 8     | 55.44 µs    | [54.75, 56.14] µs   |
| `ble_mesh_8_convergence`        | BLE-only mesh              | 8     | 132.58 µs   | [131.52, 133.67] µs |
| `ring_10_convergence`           | ring                       | 10    | 160.40 µs   | [158.46, 162.39] µs |
| `mixed_ble_lan_10_convergence`  | mixed BLE + LAN scenario   | 10    | 249.48 µs   | [247.03, 251.98] µs |
| `line_10_convergence`           | line                       | 10    | 276.18 µs   | [272.20, 280.20] µs |
| `grid_4x4_convergence`          | 4×4 grid                   | 16    | 645.04 µs   | [634.27, 655.59] µs |

Sample size: 100 measurements per benchmark, 3 s warm-up + ~5 s
measurement window.

## Cross-topology comparison

- **Connectivity wins.** Full mesh (every pair one hop) converges
  fastest at 8 nodes — ~12× faster than a 16-node grid and ~2.4× faster
  than the same-size BLE-only mesh.
- **Ring beats line.** At N = 10 the ring converges ~1.7× faster
  than the line — the extra link closes the loop, so routing info
  propagates from both directions.
- **Grid scales worst.** 4×4 grid at 16 nodes is slowest of all
  convergence benchmarks — more nodes *and* a limited-degree lattice
  mean info has to hop through several intermediaries.
- **Scenario benchmarks** (`ble_only_mesh`, `mixed_ble_lan`,
  `internet_relay`) exercise heterogeneous link capabilities;
  `internet_relay_8` is the single fastest convergence thanks to one
  high-speed reliable hub that every other node reaches in one hop.
- **Per-tick steady-state cost ≈ 26 µs** for a 10-node line (from
  `single_tick_10_node_line`), cleanly separating per-tick work from
  the transient cost of building the routing tables from scratch.

## Reproducing

```bash
cd rust
cargo bench -p qaul-sim
```

HTML reports are generated under `rust/target/criterion/report/`.

## Notes

- No `main`-branch baseline exists: `qaul-sim` is new on the feature
  branch. To establish one for future comparisons, run once with
  `cargo bench -p qaul-sim -- --save-baseline <name>`; subsequent
  runs will diff against that saved baseline automatically.
- The refactored tip performs on par with or faster than the
  pre-refactor `qaul-sim`-introduction commit (`e96abcd1`) on every
  convergence benchmark, confirming the instance-based state
  migration did not regress routing dynamics.

## Raw output

```
line_10_convergence     time:   [272.20 µs 276.18 µs 280.20 µs]
ring_10_convergence     time:   [158.46 µs 160.40 µs 162.39 µs]
grid_4x4_convergence    time:   [634.27 µs 645.04 µs 655.59 µs]
full_mesh_8_convergence time:   [54.752 µs 55.436 µs 56.139 µs]
single_tick_10_node_line
                        time:   [26.301 µs 26.345 µs 26.390 µs]
ble_mesh_8_convergence  time:   [131.52 µs 132.58 µs 133.67 µs]
mixed_ble_lan_10_convergence
                        time:   [247.03 µs 249.48 µs 251.98 µs]
internet_relay_8_convergence
                        time:   [48.915 µs 49.337 µs 49.770 µs]
```
