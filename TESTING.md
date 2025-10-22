# Testing Guide

This document describes the testing setup for the plant evolution simulator.

## Test Structure

### Unit Tests (`src/main.rs`)

The unit tests verify core initialization logic without requiring a full graphics environment:

1. **`test_core_initialization`** - Validates that core resources (VoxelWorld, DayNightCycle, YearCycle, Statistics) can be initialized with correct default values.

2. **`test_plant_spawning_logic`** - Tests that plant genomes can be generated with valid values.

### Integration Tests (`tests/startup_test.rs`)

The integration test `test_app_starts_without_crashing` actually spawns the full application and verifies it can run for a few seconds without crashing. This test is marked with `#[ignore]` by default since it requires a display/graphics environment.

## Running Tests

### Unit Tests

To run the unit tests (works in headless environments):

```bash
cargo test --bin plant-evolution-sim
```

### Integration Tests

To run the integration test (requires graphics environment):

```bash
cargo test --test startup_test -- --ignored --nocapture
```

### All Tests

To run all tests including ignored ones:

```bash
cargo test -- --include-ignored --nocapture
```

## System Requirements

The tests require the following system dependencies to compile:

- `libasound2-dev` (ALSA sound library)
- `libudev-dev` (device management)
- `libwayland-dev` (Wayland display server)
- `libxkbcommon-dev` (keyboard handling)

On Ubuntu/Debian:
```bash
sudo apt-get install libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

## Continuous Integration

To set up CI testing, create a `.github/workflows/tests.yml` file with the necessary system dependencies installation steps for Ubuntu (libasound2-dev, libudev-dev, libwayland-dev, libxkbcommon-dev).

## Test Philosophy

The tests are designed to catch startup crashes and basic initialization issues. The unit tests verify core logic can initialize, while the integration test ensures the full application can start up and run for a few frames without panicking.
