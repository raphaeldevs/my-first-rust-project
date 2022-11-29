use std::fs::{read_to_string, File};
use std::io::Read;
use std::str::from_utf8;

const FILE_PATH: &str = "hello.txt";

fn low_level_read_file() {
    let mut file = match File::open(FILE_PATH) {
        Err(err) => panic!("Couldn't read file: {}", err.to_string()),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err.to_string()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err.to_string()),
        Ok(_) => (),
    };

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.to_string()),
        Ok(value) => value,
    };

    println!("File contents, but in low-level way:\n{}", data)
}

fn high_level_read_file() {
    let file_content = match read_to_string(FILE_PATH) {
        Err(err) => panic!("Couldn't read file: {}", err.to_string()),
        Ok(data) => data,
    };

    print!("File contents, but in high-level way:\n{}", file_content)
}

fn main() {
    low_level_read_file();

    println!("--------------------------------------------------");
    println!();

    high_level_read_file();
}
