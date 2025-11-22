# Quick Start: Building and Testing the GUI

## For Developers

### Prerequisites

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libsoup-3.0-dev
```

#### macOS
```bash
# No additional dependencies needed
```

#### Windows
```bash
# Ensure WebView2 is installed (usually comes with Windows 10/11)
# Download from: https://developer.microsoft.com/microsoft-edge/webview2/
```

### Build

```bash
# Clone repository
git clone https://github.com/coryzibell/speedo
cd speedo

# Build CLI only (fast, no dependencies)
cargo build --release

# Build with GUI (requires system dependencies above)
cargo build --release --features gui
```

### Run

```bash
# CLI mode
./target/release/speedo

# GUI mode
./target/release/speedo --gui

# Or with cargo
cargo run --features gui -- --gui
```

### Development

```bash
# Watch mode for GUI development
cargo watch -x 'run --features gui -- --gui'

# Check without building
cargo check --features gui

# Run tests
cargo test
cargo test --features gui
```

### Testing the GUI

1. **Launch the GUI**: `speedo --gui`
2. **Select a server**: Click on any server in the list
3. **Run test**: Click "Run Speed Test" button
4. **View results**: Results appear below after test completes

### Modifying the GUI

#### Change styling
Edit `assets/main.css` and rebuild:
```bash
cargo build --features gui
```

#### Change layout/components
Edit `src/gui.rs` in the `app()` function:
```rust
rsx! {
    // Your component structure here
}
```

#### Add new functionality
1. Update state in the component
2. Add new UI elements in RSX
3. Wire up event handlers
4. Test with `cargo run --features gui -- --gui`

### Common Issues

**Linux: Cannot find webkit2gtk**
```bash
# Make sure you installed the -dev packages
sudo apt-get install libwebkit2gtk-4.1-dev
```

**Build takes forever**
```bash
# First build compiles all dependencies
# Use cargo-binstall for faster dependency installation
cargo install cargo-binstall
cargo binstall speedo --features gui
```

**GUI doesn't start**
```bash
# Check if you built with GUI feature
./target/release/speedo --help | grep gui

# Should show: -g, --gui    Launch graphical user interface
```

### Project Structure

```
speedo/
├── src/
│   ├── main.rs          # Entry point, CLI arg parsing
│   ├── gui.rs           # GUI module (Dioxus components)
│   ├── config.rs        # Configuration loading
│   ├── servers.rs       # Server list management
│   ├── downloader.rs    # HTTP download engine
│   ├── output.rs        # JSON/CSV formatting
│   └── ui.rs            # Terminal UI (inquire menus)
├── assets/
│   └── main.css         # GUI styling
├── Cargo.toml           # Dependencies and features
└── GUI.md               # Full GUI documentation
```

### Contributing

To contribute GUI improvements:

1. Fork the repository
2. Create feature branch: `git checkout -b feature/gui-improvement`
3. Make changes to `src/gui.rs` or `assets/main.css`
4. Test: `cargo run --features gui -- --gui`
5. Commit: `git commit -am 'Add GUI improvement'`
6. Push: `git push origin feature/gui-improvement`
7. Open pull request

### Performance Tips

- GUI shares the same async download engine as CLI
- WebView rendering is hardware-accelerated
- State updates are reactive (only re-render changed components)
- CSS is embedded at compile time (no runtime loading)

### Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run --features gui -- --gui

# Build in debug mode (faster compilation)
cargo build --features gui

# Run with backtrace on panic
RUST_BACKTRACE=1 cargo run --features gui -- --gui
```

### Resources

- **Dioxus Docs**: https://dioxuslabs.com/learn/0.6/
- **RSX Syntax**: https://dioxuslabs.com/learn/0.6/reference/rsx
- **Speedo Repo**: https://github.com/coryzibell/speedo

### Next Steps

After getting the GUI running:

1. Read `GUI_DESIGN.md` for visual design guidelines
2. Check `IMPLEMENTATION.md` for architecture overview
3. Review `ROADMAP.md` for planned GUI features
4. Join discussions for feature requests

Happy coding! 🚀
