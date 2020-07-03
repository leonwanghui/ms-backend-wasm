use std::{path::PathBuf, process::Command};

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("lib");

    if !out_dir.is_dir() {
        std::fs::create_dir(&out_dir).unwrap();
    }

    Command::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tools/build_ops_lib.py"
    ))
    .arg(&out_dir)
    .output()
    .expect("Failed to execute command!");

    let ar = option_env!("LLVM_AR").unwrap_or("llvm-ar-10");
    let add_obj_file = out_dir.join("add.o");
    let sub_obj_file = out_dir.join("sub.o");
    let lib_file = out_dir.join("libops_wasm32.a");
    Command::new(ar)
        .arg("rcs")
        .arg(&lib_file)
        .arg(&add_obj_file)
        .arg(&sub_obj_file)
        .output()
        .expect("Failed to execute command!");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
