use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read(path: &str) -> String {
    let prepend: String;
    let env_name = "AOC_INPUT";
    match env::var(env_name) {
        Ok(v) => { prepend = v }
        Err(e) => panic!("${} is not set ({})", env_name, e)
    }

    let file = File::open(prepend + path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}