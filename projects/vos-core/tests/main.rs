#[test]
fn ready() {
    println!("it works!")
}

use openapiv3::OpenAPI;

#[test]
fn main() {
    let data = include_str!("openapi31.json");
    let openapi: OpenAPI = json5::from_str(data).expect("Could not deserialize input");
    println!("{:?}", openapi);
}
