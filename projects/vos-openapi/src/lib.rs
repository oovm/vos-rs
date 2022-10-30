use openapiv3::OpenAPI;

mod visitor;
#[test]
fn main() {
    let data = include_str!("openapi31.json");
    let openapi: OpenAPI = json5::from_str(data).expect("Could not deserialize input");
    let cvt = FromOpenAPI {};
    cvt.convert(&openapi);
}

pub struct FromOpenAPI {}
