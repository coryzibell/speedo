# speedrun

Network speed test tool.

## SYNOPSIS

```
speedrun [-i|--interactive] [-n|--non-interactive]
speedrun --help
speedrun --version
```

## DESCRIPTION

speedrun downloads a test file and reports the transfer speed. By default it runs non-interactively against Cloudflare's CDN. With the -i flag, it displays a menu for selecting different test servers.

Command-line flags override the config file setting.

## OPTIONS

**-i, --interactive**
    Show server selection menu

**-n, --non-interactive**
    Run quick test (override config)

**-h, --help**
    Display help text

**-V, --version**
    Display version

## CONFIGURATION

Configuration is read from the first file found:
- ./speedrun.toml
- ./.speedrun.toml
- ~/.speedrun.toml

### Example Configuration

```toml
# Default mode when no flags given (default: false)
interactive = false

# User agent string for requests
user_agent = "Mozilla/5.0"

# Additional test servers
[[custom_servers]]
name = "My Server"
url = "https://example.com/testfile.bin"
```

See speedrun.toml.example for details.

## OUTPUT

Non-interactive mode prints only the speed:
```
44.60 MB/s  (374.14 Mbps)
```

Interactive mode displays:
- HTTP status code
- Connection time
- Time to first byte (TTFB)
- Total transfer time
- File size
- Transfer speed

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

- speedrun.toml - configuration file
- ~/.speedrun.toml - user configuration file

## SEE ALSO

curl(1), wget(1)

