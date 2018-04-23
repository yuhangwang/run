use std::fs;
use std::path::PathBuf;

fn main() {
    let dir: PathBuf = PathBuf::from("./src");
    println!("{:?}", fs::canonicalize(&dir).unwrap());

    let dir2 = PathBuf::from("../../src");
    println!("{:?}", fs::canonicalize(&dir2).unwrap());
}
