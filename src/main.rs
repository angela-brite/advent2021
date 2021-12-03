use std::env;
use anyhow::Error;
mod common;
mod days;
use crate::days::*;

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    let selection = args[1].parse::<i32>()?;

    match selection {
        1 => day1::run(),
        _ => println!("Unknown day provided!"),
    };
    Ok(())
}
