// This module embeds a selection of .flf font files for ASCII art rendering.
// Add or remove fonts as desired for binary size and style variety.

include!(concat!(env!("OUT_DIR"), "/generated_fonts.rs"));

pub fn decompress_font(data: &[u8]) -> anyhow::Result<Vec<u8>> {
    zstd::stream::decode_all(data).map_err(|e| anyhow::anyhow!(e))
}
