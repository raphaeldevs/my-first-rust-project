use std::fs::File;
use std::io::Read;
use std::str::from_utf8;

fn main() {
    let file_path = "hello.txt";

    let mut file = match File::open(file_path) {
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

    print!("{}", data)
}
