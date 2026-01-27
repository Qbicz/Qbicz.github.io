# kubicz.engineer

My personal website built with Rust.

## Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add wasm target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install trunk
```

### Run locally

```bash
trunk serve
```

Opens at http://localhost:8080

### Build

```bash
trunk build --release
```

Output in `dist/` folder.

## Deployment

### Automatic (Recommended)

Push to `master` branch → GitHub Actions CI builds and deploys automatically.

### Manual

```bash
./deploy.sh
```

## Tech Stack

- **Leptos** - Rust web framework (CSR mode)
- **WebAssembly** - Compiled Rust runs in browser
- **Trunk** - Build tool & bundler
- **GitHub Pages** - Static hosting
