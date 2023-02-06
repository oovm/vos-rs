use diagnostic_quick::QResult;
use peginator_codegen::Compile;
use std::{env::current_dir, path::PathBuf};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
#[ignore]
fn build_parser() -> QResult {
    let path = PathBuf::from(current_dir().unwrap());
    let output = path.join("src/parser/vos.rs");
    Compile::file("src/parser/vos.peg").destination(output).format().run().unwrap();
    Ok(())
}
