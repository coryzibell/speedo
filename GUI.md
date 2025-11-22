# Building Speedo with GUI Support

## GUI Feature

Speedo includes an optional graphical user interface (GUI) built with **Freya**. Freya is a native GUI framework that uses **Skia** for rendering, providing smooth 60 FPS performance without the overhead of web technologies.

## Requirements

### All Platforms
Freya uses native rendering (Skia), so there are **no system webview dependencies** required!

The only requirements are:
- **Rust 1.70+**
- Standard build tools (gcc/clang)

### Platform-Specific Notes

**Linux:**
- May need basic X11/Wayland libraries (usually already installed)
- `sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev` (if needed)

**macOS:**
- No additional dependencies

**Windows:**
- No additional dependencies

## Building

### Build with GUI support:
```bash
cargo build --release --features gui
```

### Build without GUI (CLI only - default):
```bash
cargo build --release
```

## Running the GUI

After building with the `gui` feature:

```bash
# Run with GUI
speedo --gui
speedo -g

# Or from cargo
cargo run --features gui -- --gui
```

## Features

The GUI provides:
- **Server selection** - Browse and select from 70+ speed test servers
- **One-click testing** - Simple button to run speed tests
- **Real-time results** - See download speeds, latency, and connection stats
- **Native performance** - 60 FPS Skia rendering, no web overhead
- **Dark theme** - Modern UI with smooth animations

## Why Freya?

**Freya** was chosen over web-based solutions because:

1. **Native Performance** - Skia GPU-accelerated rendering
2. **No Dependencies** - No WebView, WebKit, or Chromium required
3. **Small Binary** - Significantly smaller than Electron or Tauri
4. **Cross-Platform** - Same code works on Linux, macOS, Windows
5. **Rust-Native** - Built specifically for Rust applications

## CLI vs GUI

You can use either interface based on your preference:

- **CLI (Terminal)** - Lightweight, scriptable, no compilation dependencies
  ```bash
  speedo                    # Quick test
  speedo -i                 # Interactive menu
  speedo --json             # Machine-readable output
  ```

- **GUI (Desktop App)** - Native visual interface with Skia rendering
  ```bash
  speedo --gui
  ```

## Technical Details

- Built with [Freya](https://freyaui.dev/) - Native Rust GUI framework
- Uses [Skia](https://skia.org/) - Google's 2D graphics library
- Fully asynchronous with Tokio runtime
- Shares core download engine with CLI for consistent results
- No HTML, CSS, or JavaScript involved

## Troubleshooting

**Build errors on Linux**
```bash
# Install X11 development libraries if needed
sudo apt-get install libxcb-shape0-dev libxcb-xfixes0-dev
```

**Build time**
- First build with GUI feature may take 3-5 minutes (compiling Skia)
- Subsequent builds are much faster
- Much faster than webkit2gtk-based solutions!

**Runtime performance**
- 60 FPS smooth animations
- Low memory usage (~30-40MB)
- Native look and feel
