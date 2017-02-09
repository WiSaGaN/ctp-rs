extern crate bindgen;
extern crate encoding;

use encoding::{ decode, DecoderTrap };
use encoding::all::GB18030;
use std::io::{ Read, Write };
use std::path::Path;

pub fn gb18030_bytes_to_string(bytes: &[u8]) -> String {
    decode(bytes, DecoderTrap::Replace, GB18030).0.unwrap_or_else(|e| e.into_owned())
}

fn generate_data_type(input_h: &Path, output_rs: &Path) -> Result<(), String> {
    let mut file_bytes = vec!();
    let mut input_file = try!(std::fs::File::open(input_h).map_err(|e| format!("failed to open data_type header, {}", e)));
    try!(input_file.read_to_end(&mut file_bytes).map_err(|e| format!("filed to read data_type header, {}", e)));
    let mut type_output = std::io::BufWriter::new(try!(std::fs::File::create(output_rs).map_err(|e| format!("cannot create data_type file, {}", e))));
    let file_string = gb18030_bytes_to_string(&file_bytes);
    for line in file_string.lines() {
        if line.starts_with("#define") {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() == 3 {
                let name = unsafe { tokens.get_unchecked(1) };
                let value = unsafe { tokens.get_unchecked(2) };
                if value.starts_with("'") && value.ends_with("'") && value.len() == 3 {
                    try!(type_output.write(format!("pub const {}: u8 = b{};\n", name, value).as_bytes()).map_err(|e| format!("cannot write data_type file, {}", e)));
                }
            }
        }
    }
    Ok(())
}

fn generate_struct(input_h: &Path, output_rs: &Path) -> Result<(), String> {
    let binding = try!(bindgen::builder().header(input_h.to_string_lossy().into_owned()).generate().map_err(|_| format!("failed to generate binding" )));
    let binding_output = binding.to_string().replace("c_char", "c_uchar");
    let mut output_file = try!(std::fs::File::create(output_rs).map_err(|e| format!("cannot create struct file, {}", e)));
    output_file.write_all(binding_output.as_bytes()).map_err(|e| format!("cannot write struct file, {}", e))
}

fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let data_type_header = "../api-ctp/include/ThostFtdcUserApiDataType.h";
    let data_type_out_path = format!("{}/data_type.rs.in", out_dir);
    let struct_header = "../api-ctp/include/ThostFtdcUserApiStruct.h";
    let struct_out_path = format!("{}/struct.rs.in", out_dir);
    generate_data_type(Path::new(data_type_header), Path::new(&data_type_out_path)).unwrap();
    generate_struct(Path::new(struct_header), Path::new(&struct_out_path)).unwrap();
}
