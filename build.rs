use std::{env, path::PathBuf};

fn main() {
    let qat_path =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"))
            .join("tree-sitter-qat");
    cc::Build::new()
        .include(qat_path.join("src"))
        .file(qat_path.join("src").join("parser.c"))
        .compile("tree-sitter-qat");
}
