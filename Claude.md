# Claude Development Guidelines

## Testing

**IMPORTANT**: Tests should always target wasm (WebAssembly).

### Running Wasm Tests

To verify tests compile for wasm:
```bash
cargo test --target wasm32-unknown-unknown
```

This ensures that all tests are compatible with the wasm build target and helps catch platform-specific issues early.

### Setup

The wasm32-unknown-unknown target has been installed. If you need to reinstall it:
```bash
rustup target add wasm32-unknown-unknown
```

### Current Test Status

All tests successfully compile for the wasm target as of 2025-10-23. The compilation validates that:
- Test code is wasm-compatible
- Dependencies support wasm
- No platform-specific code breaks in wasm context
