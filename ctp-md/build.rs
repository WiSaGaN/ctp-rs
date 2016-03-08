fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let current_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let so_symlink_string = format!("{}/libthostmduserapi.so", out_dir);
    let so_symlink = std::path::Path::new(&so_symlink_string);
    if so_symlink.exists() {
        std::fs::remove_file(so_symlink).expect("symlink exists, but failed to remove it");
    }
    std::os::unix::fs::symlink(&format!("{}/../api-ctp/lib/thostmduserapi.so", current_dir), so_symlink).expect("failed to create new symlink");
    println!("cargo:rustc-link-search=native={}", out_dir);
}
