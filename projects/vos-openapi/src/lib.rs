use std::{fs::File, io::Write};

use openapiv3::OpenAPI;

mod visitor;
#[test]
fn main() {
    let data = include_str!("openapi31.json");
    let openapi: OpenAPI = json5::from_str(data).expect("Could not deserialize input");
    let cvt = FromOpenAPI {};
    let out = cvt.convert(&openapi);
    let out = json5::to_string(&out).unwrap();
    let mut file = File::create("src/openapi31.vos").unwrap();
    file.write_all(out.as_bytes()).unwrap();
}

pub struct FromOpenAPI {}
