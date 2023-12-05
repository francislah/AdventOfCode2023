use std::fs::File;
use std::io::{BufRead, BufReader};
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

pub fn read_file(reader: BufReader<File>) {
    reader.lines().for_each(|line| match line {
        Ok(line) => println!("{}", line),
        Err(e) => println!("Error: {}", e)
    })
}

pub fn read_file_into_vec(file: &File) -> Vec<String> {
    let mut lines: Vec<String> = vec![];
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| match line {
        Ok(line) => lines.push(line),
        Err(e) => println!("Error: {}", e)
    });
    lines
}