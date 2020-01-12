//! Rust script used for generate sources in `src/generated`
//! Use `cargo script` to run it
//!
//! ```cargo
//! [package]
//! edition = "2018"
//!
//! [dependencies]
//! bindgen = "0.30"
//! encoding = "0.2"
//! xmltree = "0.4"
//! ```
use encoding::{ decode, DecoderTrap };
use encoding::all::GB18030;
use std::io::{ Read, Write };
use std::path::Path;
use xmltree as xml;

pub fn gb18030_bytes_to_string(bytes: &[u8]) -> String {
    decode(bytes, DecoderTrap::Replace, GB18030).0.unwrap_or_else(|e| e.into_owned())
}

fn generate_struct(input_h: &str, output_rs: &str) -> Result<(), String> {
    let binding = bindgen::builder()
                       .header(input_h)
                       .derive_debug(false)
                       .derive_default(true)
                       .generate()
                       .map_err(|_| format!("failed to generate binding" ))?;
    let binding_output = binding.to_string().replace("c_char", "c_uchar");
    let mut output_file = std::fs::File::create(output_rs).map_err(|e| format!("cannot create struct file, {}", e))?;
    output_file.write_all(binding_output.as_bytes()).map_err(|e| format!("cannot write struct file, {}", e))
}

#[derive(Debug)]
struct ErrorEntry {
    id: String,
    value: i64,
    prompt: String,
}

#[derive(Debug)]
struct Errors {
    errors: Vec<ErrorEntry>,
}

impl Errors {
    pub fn from_xml_element(element: xml::Element) -> Result<Self, String> {
        let mut errors = Vec::new();
        for child in element.children {
            let id = child.attributes.get("id").ok_or(String::from("no id attribute in one of the child"))?.to_owned();
            let value_string = child.attributes.get("value").ok_or(String::from("no value attribute in one of the child"))?;
            let value = value_string.parse().map_err(|e| format!("cannot parse value to integer, {}", e))?;
            let prompt = child.attributes.get("prompt").ok_or(String::from("no prompt attribute in one of the child"))?.to_owned();
            errors.push(ErrorEntry {
                id: id,
                value: value,
                prompt: prompt,
            });
        }
        Ok(Errors {
            errors: errors,
        })
    }
}

fn generate_error(input_xml: &Path, output_rs: &Path) -> Result<(), String> {
    let mut file_bytes = vec!();
    let mut input_file = std::fs::File::open(input_xml).map_err(|e| format!("failed to open data_type header, {}", e))?;
    input_file.read_to_end(&mut file_bytes).map_err(|e| format!("filed to read data_type header, {}", e))?;
    let file_string = gb18030_bytes_to_string(&file_bytes);
    let element = xml::Element::parse(file_string.as_bytes()).map_err(|e| format!("failed to parse input file as xml, {}", e))?;
    let errors = Errors::from_xml_element(element).map_err(|e| format!("cannot generate errors from parsed xml element, {}", e))?;
    let mut error_output = std::io::BufWriter::new(std::fs::File::create(output_rs).map_err(|e| format!("cannot create error file, {}", e))?);

    for error in errors.errors.iter() {
        error_output.write(format!("pub const ERROR_{}: TThostFtdcErrorIDType = {};\n", error.id, error.value).as_bytes()).map_err(|e| format!("cannot write error file, {}", e))?;
    }
    error_output.write(format!("pub fn error_id_to_chinese_description(error_id: TThostFtdcErrorIDType) -> &'static str {{\n").as_bytes()).map_err(|e| format!("cannot write error file, {}", e))?;
    error_output.write(format!("    match error_id {{\n").as_bytes()).map_err(|e| format!("cannot write error file, {}", e))?;
    for error in errors.errors.iter() {
        error_output.write(format!("        ERROR_{} => \"{}\",\n", error.id, error.prompt).as_bytes()).map_err(|e| format!("cannot write error file, {}", e))?;
    }
    error_output.write(format!("        _ => \"unknown error\",\n").as_bytes()).map_err(|e| format!("cannot write error file, {}", e))?;
    error_output.write(format!("    }}\n}}\n").as_bytes()).map_err(|e| format!("cannot write error file, {}", e))?;
    Ok(())
}

fn main() {
    let out_dir = "./src/generated";
    let struct_header = "../api-ctp/include/ThostFtdcUserApiStruct.h";
    let struct_out_path = format!("{}/struct.rs.in", out_dir);
    generate_struct(struct_header, &struct_out_path).unwrap();
    let error_xml = "../api-ctp/misc/error.xml";
    let error_out_path = format!("{}/error.rs.in", out_dir);
    generate_error(Path::new(error_xml), Path::new(&error_out_path)).unwrap();
}
