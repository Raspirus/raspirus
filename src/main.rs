use crate::backend::file_scanner;
mod backend;

fn main() {
    println!("Hello, world!");
    let path = String::from("./backend");
    let fscanner = file_scanner::FileScanner::new(path).unwrap();

    fscanner.search_files();
}

