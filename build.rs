#![warn(missing_docs)]

extern crate cc;
extern crate serde;

use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::io;

#[derive(Deserialize)]
struct Config {
    llvm_path: String
}

fn main() -> io::Result<()> {
    let config: Config = serde_yaml::from_reader(BufReader::new(File::open("config.yml")?))
        .expect("error | InvalidYamlFile");
    let result = cc::Build::new()
        .cpp(true)
        .warnings(true)
        .debug(true)
        .file("src/main.cpp")
        .include(&config.llvm_path)
        .include("src")
        .compile("cpp");
    Ok(result)
}