# Performance Benchmarks

This directory contains performance benchmarks for comparing development and production modes.

## Benchmark Coverage

- Production mode vs hand-written baseline
- Hot-reload latency measurements
- XML parsing performance
- Widget tree construction overhead
- Memory usage profiling

## Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench --bench prod_mode_bench
```

## Performance Targets

From spec requirements:
- Production mode: <5% overhead vs hand-written code
- Hot-reload: <300ms total latency
- File change detection: <100ms
- Startup time: <50ms for 1000 widget UI

## Adding New Benchmarks

Benchmarks should use the `criterion` crate and follow the existing patterns.
