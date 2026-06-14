# UniLang WebAssembly REPL

A web-based REPL (Read-Eval-Print Loop) for the UniLang command framework, compiled to WebAssembly.

## Features

- **WebAssembly Performance**: Native Rust performance in the browser
- **Interactive REPL**: Real-time command execution and feedback
- **Cross-Platform Validation**: Works consistently across all platforms
- **SIMD Optimizations**: Fast parsing and tokenization (when available)
- **Modern UI**: Dark theme with command history (arrow keys)

## Project Structure

```
examples/wasm-repl/
├── Cargo.toml          # WebAssembly-specific dependencies
├── src/
│   └── lib.rs          # Rust/WASM bridge implementation
├── www/                # Web frontend
│   ├── index.html      # Main HTML interface
│   ├── style.css       # Modern dark theme styles
│   └── bootstrap.js    # JavaScript WASM loader
├── tests/              # WASM integration tests
│   └── wasm.rs         # Browser-side WASM tests via wasm-bindgen-test
├── pkg/                # Generated WASM bindings (after build)
└── readme.md           # This file
```

## Files

| File | Responsibility |
|------|----------------|
| `Cargo.toml` | WASM-specific dependencies and build config |
| `readme.md` | WASM REPL documentation and usage guide |
| `src/lib.rs` | Rust/WASM bridge using wasm-bindgen |
| `tests/wasm.rs` | Browser-side WASM tests |
| `www/bootstrap.js` | JavaScript WASM module loader |
| `www/index.html` | Main web interface HTML |
| `www/style.css` | Dark theme UI styles |

## Quick Start

### Prerequisites

- [wasm-pack](https://rustwasm.github.io/wasm-pack/) for building WebAssembly
- A local web server for development

### Building

1. **Build the WebAssembly module:**
   ```bash
   cd examples/wasm-repl
   wasm-pack build --target web
   ```

2. **Serve the web interface:**
   ```bash
   cd www
   python3 -m http.server 8000
   ```

3. **Open in browser:**
   Navigate to `http://localhost:8000`

## Usage Examples

```bash
# Get help
.help

# Echo text (demo command)
.demo.echo text::Hello

# Simple calculator
.calc.add a::42 b::58
```

## WebAssembly Compatibility

This example demonstrates how UniLang works in WebAssembly environments:

- **Conditional Compilation**: Filesystem operations are disabled for WASM targets
- **Minimal Dependencies**: Uses only web-compatible dependencies
- **Optimized Build**: Small binary size with `opt-level = "s"` and LTO
- **Error Handling**: Proper panic hooks for debugging

## Development

### Key Files

- **`src/lib.rs`**: Main Rust/WASM interface
- **`www/bootstrap.js`**: JavaScript bridge to WASM module
- **`Cargo.toml`**: WebAssembly-optimized dependencies

### WASM Features Used

- `wasm-bindgen` for Rust/JavaScript interop
- `web-sys` for DOM manipulation
- `js-sys` for JavaScript API access
- `console_error_panic_hook` for better debugging

### Building for Production

```bash
wasm-pack build --target web --release
ls -lh pkg/
```

## Testing

Run the WASM-specific tests:

```bash
wasm-pack test --chrome --headless
wasm-pack test --firefox --headless
```

## Known Limitations

- File system operations are not available (by design)
- Some native commands are disabled in WebAssembly mode
- Browser security restrictions apply to certain features
