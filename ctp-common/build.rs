extern crate bindgen;

use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let binding = bindgen::builder().header("../api-ctp/include/ThostFtdcUserApiStruct.h").generate().expect("failed to generate binding");
    let out_path = format!("{}/ctp-api.rs.in", out_dir);
    let binding_output = binding.to_string().replace("c_char", "c_uchar");
    std::fs::File::create(out_path).expect("can't create file for binding output").write_all(binding_output.as_bytes()).expect("can't write binding output to file");
    let so_symlink_string = format!("{}/libthostmduserapi.so", out_dir);
    let so_symlink = std::path::Path::new(&so_symlink_string);
    if so_symlink.exists() {
        std::fs::remove_file(so_symlink).expect("symlink exists, but failed to remove it");
    }
    std::os::unix::fs::symlink(&format!("{}/../api-ctp/lib/thostmduserapi.so", current_dir), so_symlink).expect("failed to create new symlink");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
