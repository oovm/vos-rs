use json5::from_str;
use std::{fs::File, io::Write};

use openapiv3::OpenAPI;
use vos_core::Parser;

mod visitor;
#[test]
fn main() {
    let data = include_str!("openapi31.json");
    let openapi: OpenAPI = from_str(data).expect("Could not deserialize input");
    let cvt = FromOpenAPI {};
    let out = cvt.parse(&openapi).unwrap();
    // let out = json5::to_string(&out).unwrap();
    let mut file = File::create("src/openapi31.vos").unwrap();
    file.write_all(out.to_string().as_bytes()).unwrap();
}

pub struct FromOpenAPI {}
