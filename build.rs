use std::env;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_fonts.rs");
    let fonts_dir = Path::new("fonts");
    let compressed_dir = Path::new(&out_dir).join("compressed_fonts");
    std::fs::create_dir_all(&compressed_dir)?;
    let mut file = File::create(&dest_path)?;

    writeln!(file, "use rand::seq::IteratorRandom;")?;
    writeln!(file, "pub fn get_embedded_fonts() -> Vec<(&'static str, &'static [u8])> {{")?;
    writeln!(file, "    vec![")?;

    let mut font_files: Vec<_> = fs::read_dir(fonts_dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().extension().map(|e| e == "flf").unwrap_or(false))
        .collect();
    font_files.sort_by_key(|e| e.file_name());

    for entry in font_files {
        let name = entry.file_name().into_string().unwrap();
        let font_path = entry.path();
        let mut font_data = Vec::new();
        File::open(&font_path)?.read_to_end(&mut font_data)?;
        let compressed_path = compressed_dir.join(format!("{}.zst", name));
        let mut encoder = zstd::Encoder::new(File::create(&compressed_path)?, 0).unwrap();
        encoder.write_all(&font_data).unwrap();
        encoder.finish().unwrap();
        writeln!(
            file,
            "        (\"{}\", include_bytes!(concat!(env!(\"OUT_DIR\"), \"/compressed_fonts/{}.zst\"))),",
            name, name
        )?;
    }

    writeln!(file, "    ]")?;
    writeln!(file, "}}")?;
    writeln!(file, "\npub fn random_font() -> (&'static str, &'static [u8]) {{")?;
    writeln!(file, "    let fonts = get_embedded_fonts();")?;
    writeln!(file, "    let mut rng = rand::rng();")?;
    writeln!(file, "    fonts.into_iter().choose(&mut rng).unwrap_or((\"Standard.flf\", include_bytes!(concat!(env!(\"OUT_DIR\"), \"/compressed_fonts/Standard.flf.zst\"))))")?;
    writeln!(file, "}}")?;
    Ok(())
}
