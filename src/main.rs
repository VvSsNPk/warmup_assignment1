use aisysproj::directory_parser;
use std::path::PathBuf;

pub mod state;

fn main() {
    let mut store = String::new();
    println!("Give the Name of the Directory which contains all the file of the assignment : ");
    std::io::stdin().read_line(&mut store).expect("Invalid File Name");
    let mut path = PathBuf::new();
    path.push(store.trim());
    directory_parser(&mut path);
}
