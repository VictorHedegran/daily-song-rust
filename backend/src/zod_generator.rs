use std::fs;
use std::path::Path;
use zod_gen::ZodGenerator;

use crate::models::TracksPage;

pub fn generate_types() {
    let mut generator = ZodGenerator::new();

    // Add all your types with meaningful names
    generator.add_schema::<TracksPage>("TracksPage");

    // Generate a single TypeScript file with all schemas
    let content = generator.generate();
    let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("frontend/src/lib/types/schemas.ts");
    fs::write(&output_path, content).unwrap();
}
