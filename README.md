# speedo

Network speed test tool and file downloader. Built in rust, doesn't need curl, wget, or system ssl libs.

## Installation

### From crates.io

```bash
cargo install speedo
```

### Using cargo-binstall

```bash
cargo binstall speedo
```

### From source

```bash
cargo install --git https://github.com/coryzibell/speedo
```

## SYNOPSIS

```
speedo [URL]
speedo [-i|--interactive] [-n|--non-interactive] [-s|--speed-unit UNIT]
speedo --help
speedo --version
```

## DESCRIPTION

speedo downloads a test file and reports the transfer speed. By default it runs non-interactively against Cloudflare's CDN. With the -i flag, it displays a menu for selecting different test servers.

If a URL is provided as an argument, the file is downloaded to the current directory and the speed is reported.

Command-line flags override the config file settings.

## OPTIONS

**-i, --interactive**
    Show server selection menu

**-n, --non-interactive**
    Run quick test (override config)

**-s, --speed-unit UNIT**
    Speed unit format (bits-metric, bits-binary, bytes-metric, bytes-binary)

**-h, --help**
    Display help text

**-V, --version**
    Display version

## ARGUMENTS

**URL**
    URL to download (saves file to current directory)

## CONFIGURATION

Configuration is read from the first file found:
- ./speedo.toml
- ./.speedo.toml
- ~/.speedo.toml

### Example Configuration

```toml
# Default mode when no flags given (default: false)
interactive = false

# User agent string for requests
user_agent = "Mozilla/5.0"

# Speed unit format for progress display (default: "bytes-metric")
# Options:
#   "bits-metric" or "mbps"  - Megabits per second (Mbps, Gbps) - 1000-based
#   "bits-binary" or "mibps" - Mebibits per second (Mibps, Gibps) - 1024-based
#   "bytes-metric" or "mb/s" - Megabytes per second (MB/s, GB/s) - 1000-based (default)
#   "bytes-binary" or "mib/s" - Mebibytes per second (MiB/s, GiB/s) - 1024-based
speed_unit = "bytes-metric"

# Additional test servers
[[custom_servers]]
name = "My Server"
url = "https://example.com/testfile.bin"
```

See speedo.toml.example for details.

## EXAMPLES

Run a quick speed test (default server):
```
speedo
```

Download a specific file:
```
speedo https://example.com/testfile.zip
```

Show interactive menu:
```
speedo -i
```

Force non-interactive mode (override config):
```
speedo -n
```

Use bits per second instead of bytes:
```
speedo --speed-unit bits-metric
```

Download with custom speed unit:
```
speedo -s bits-binary https://example.com/file.bin
```

## OUTPUT

Non-interactive mode prints the transfer summary and speed:
```
Downloaded 95.37 MiB in 4.69s - 20.33 MB/s  (170.58 Mbps)
```

When downloading a URL, the saved filename is also printed:
```
Downloaded 95.37 MiB in 4.69s - 20.33 MB/s  (170.58 Mbps)
Saved: testfile.zip
```

Interactive mode displays a progress bar during download, then shows:
- Transfer summary (size and time)
- HTTP status code
- Connection time
- Time to first byte (TTFB)
- Total transfer time
- File size
- Transfer speed

### Speed Unit Configuration

You can configure the speed display format in speedo.toml:
- **bits-metric** - Mbps, Gbps (megabits, gigabits per second - 1000-based)
- **bits-binary** - Mibps, Gibps (mebibits, gibibits per second - 1024-based)
- **bytes-metric** - MB/s, GB/s (megabytes, gigabytes per second - 1000-based) - default
- **bytes-binary** - MiB/s, GiB/s (mebibytes, gibibytes per second - 1024-based)

## SERVERS

Pre-configured test servers:
- Cloudflare (CDN) - default
- Tele2 (Global)
- Hetzner (Nuremberg, Falkenstein, Helsinki, Ashburn, Hillsboro, Singapore)
- Vultr (New Jersey, Silicon Valley, Singapore)

## BUILDING

```
cargo build --release
```

## FILES

- speedo.toml - configuration file
- ~/.speedo.toml - user configuration file

## SEE ALSO

curl(1), wget(1)

## LICENSE

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

