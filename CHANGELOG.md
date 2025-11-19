# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-11-18

### Added
- Real-time download speed display during transfer with configurable units
- `speed_unit` configuration option in speedo.toml (bits-metric, bits-binary, bytes-metric, bytes-binary)
- `-s, --speed-unit` command-line flag to override speed unit format
- Transfer summary showing total size and time after download completes
- Progress bar now displays current download speed in real-time (updates every 100ms)
- Support for displaying speeds in bits (Mbps, Mibps) or bytes (MB/s, MiB/s)
- Support for metric (1000-based) or binary (1024-based) unit calculations
- `bytesize` crate for proper byte size formatting

### Changed
- Progress display no longer shows "/0 B" for unknown file sizes (streaming downloads)
- Progress bar uses spinner for unknown file sizes instead of empty progress bar
- Output format now shows "Downloaded X MiB in Y.ZZs" for all modes
- Default speed unit is bytes-metric (MB/s) matching common expectations

### Improved
- Download speed calculation more accurate with 100ms sampling intervals
- Better handling of downloads without Content-Length header
- Documentation updated across README, --help, and speedo.toml.example

## [0.2.12] - Previous releases
- See git history for earlier changes
