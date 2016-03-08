extern crate bindgen;

use std::io::Write;

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let binding = bindgen::builder().header("../api-ctp/include/ThostFtdcUserApiStruct.h").generate().expect("failed to generate binding");
    let binding_output = binding.to_string().replace("c_char", "c_uchar");
    let out_path = format!("{}/ctp-api.rs.in", out_dir);
    std::fs::File::create(out_path).expect("can't create file for binding output").write_all(binding_output.as_bytes()).expect("can't write binding output to file");
}
