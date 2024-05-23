use aisysproj::directory_parser;
use std::path::PathBuf;

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problems");
    directory_parser(&mut path);
}
