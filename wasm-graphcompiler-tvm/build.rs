use std::path::PathBuf;

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("lib");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
