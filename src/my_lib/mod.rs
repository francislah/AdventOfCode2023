use std::fs::File;
use std::process;

pub fn open_file(filepath: &str) -> File {
    match File::open(filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            process::exit(1)
        }
    }
}
