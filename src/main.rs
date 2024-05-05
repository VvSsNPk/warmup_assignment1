use std::path::{PathBuf};
use aisysproj::directory_parser;

pub mod state;

fn main() {
    let mut path = PathBuf::new();
    path.push("problems");
    directory_parser(&mut path);
}
