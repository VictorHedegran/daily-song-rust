use std::fs;
use std::path::Path;
use zod_gen::ZodGenerator;

use crate::models::{Track, TracksPage};

pub fn generate_types() {
    let mut generator = ZodGenerator::new();

    generator.add_schema::<TracksPage>("TracksPage");
    generator.add_schema::<Track>("Track");

    let content = generator.generate();
    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("frontend/src/lib/types/schemas.ts");
    fs::write(&output_path, content).unwrap();
}
