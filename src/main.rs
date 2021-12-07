use std::env;
use anyhow::Error;
mod common;
mod days;
use crate::days::*;

fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    let selection = args[1].parse::<u32>()?;
    let part = args.get(2).unwrap_or(&("1".to_string())).parse::<u32>()?;

    match selection {
        1 => day1::run(part),
        2 => day2::run(part),
        _ => println!("Unknown day provided!"),
    };
    Ok(())
}
