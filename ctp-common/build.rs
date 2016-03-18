extern crate bindgen;
extern crate encoding;

use encoding::{ decode, DecoderTrap };
use encoding::all::GB18030;
use std::io::{ BufRead, Read, Write };

pub fn gb18030_bytes_to_string(bytes: &[u8]) -> String {
    decode(bytes, DecoderTrap::Replace, GB18030).0.unwrap_or_else(|e| e.into_owned())
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let struct_header = "../api-ctp/include/ThostFtdcUserApiStruct.h";
    let struct_out_path = format!("{}/struct.rs.in", out_dir);
    let data_type_header = "../api-ctp/include/ThostFtdcUserApiDataType.h";
    let data_type_out_path = format!("{}/data_type.rs.in", out_dir);

    let binding = bindgen::builder().header(struct_header).generate().expect("failed to generate binding");
    let binding_output = binding.to_string().replace("c_char", "c_uchar");
    std::fs::File::create(struct_out_path).expect("cannot create struct file").write_all(binding_output.as_bytes()).expect("cannot write truct file");

    let mut file_bytes = vec!();
    std::fs::File::open(data_type_header).expect("failed to open data_type header").read_to_end(&mut file_bytes).expect("filed to read data_type header");
    let mut type_output = std::io::BufWriter::new(std::fs::File::create(data_type_out_path).expect("cannot create data_type file"));
    let file_string = gb18030_bytes_to_string(&file_bytes);
    for line in file_string.lines() {
        if line.starts_with("#define") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() == 3 {
                let name = unsafe { tokens.get_unchecked(1) }.to_uppercase();
                let value = unsafe { tokens.get_unchecked(2) };
                if value.starts_with("'") && value.ends_with("'") && value.len() == 3 {
                    type_output.write(format!("pub const {}: u8 = b{};\n", name, value).as_bytes()).expect("cannot write data_type file");
                }
            }
        }
    }
}
