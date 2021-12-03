
use std::fs;

pub fn open_file(filename: &str) -> String{
    fs::read_to_string(filename).expect("Unable to open file")
}