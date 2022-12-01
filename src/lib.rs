use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

const ENV_NAME: &str = "AOC_INPUT";
pub fn read(path: &str) -> String {
    let prepend = match env::var(ENV_NAME) {
        Ok(v) => { v }
        Err(e) => panic!("${} is not set ({})", ENV_NAME, e)
    };

    std::fs::read_to_string(prepend + path).unwrap()
}