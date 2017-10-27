extern crate memmap;
extern crate object;

use std::{env, fs, process};

fn main() {
    let arg_len = env::args().len();
    if arg_len <= 1 {
        eprintln!("Usage: {} <file> ...", env::args().next().unwrap());
        process::exit(1);
    }

    for file_path in env::args().skip(1) {
        if arg_len > 2 {
            println!("");
            println!("{}:", file_path);
        }

        let file = match fs::File::open(&file_path) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to open file '{}': {}", file_path, err,);
                return;
            }
        };
        let file = match memmap::Mmap::open(&file, memmap::Protection::Read) {
            Ok(mmap) => mmap,
            Err(err) => {
                println!("Failed to map file '{}': {}", file_path, err,);
                return;
            }
        };
        let file = match object::File::parse(unsafe { file.as_slice() }) {
            Ok(file) => file,
            Err(err) => {
                println!("Failed to parse file '{}': {}", file_path, err);
                return;
            }
        };

        for section in file.get_sections() {
            println!("{:?}", section);
        }

    }
}
