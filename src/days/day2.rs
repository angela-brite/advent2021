use anyhow::anyhow;
use std::ops::Add;
use std::str::FromStr;
use crate::common::lines_from_file;

#[derive(Debug)]
enum SubCommand {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for SubCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let vec = s
            .split_once(" ")
            .ok_or(anyhow!("Unable to parse line: {}", s))?;
        let dir = vec.0;
        let mag = vec.1.parse::<u32>()?;

        match dir {
            "forward" => Ok(SubCommand::Forward(mag)),
            "down" => Ok(SubCommand::Down(mag)),
            "up" => Ok(SubCommand::Up(mag)),
            _ => Err(anyhow!("Unable to parse line: {}", s)),
        }
    }
}

#[derive(Debug, Default)]
struct SubPosition {
    horizontal: u32,
    depth: u32,
}

impl Add<SubCommand> for SubPosition {
    type Output = Self;

    fn add(self, other: SubCommand) -> Self {
        let mut h = self.horizontal;
        let mut d = self.depth;
        match other {
            SubCommand::Forward(x) => h = h + x,
            SubCommand::Down(y) => d = d + y,
            SubCommand::Up(y) => d = d - y,
        };

        Self {
            horizontal: h,
            depth: d,
        }
    }
}

#[derive(Debug, Default)]
struct SubPositionActual {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Add<SubCommand> for SubPositionActual {
    type Output = Self;

    fn add(self, other: SubCommand) -> Self {
        let mut h = self.horizontal;
        let mut d = self.depth;
        let mut a = self.aim;  

        match other {
            SubCommand::Forward(x) => {
                h = h + x;
                d = d + a * x;
            },
            SubCommand::Down(y) => a = a + y,
            SubCommand::Up(y) => a = a - y,
        };

        Self {
            horizontal: h,
            depth: d,
            aim: a, 
        }
    }
}
pub fn run(part: u32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => eprintln!("Unknown part given!")
    }
}

pub fn part_one() {
    let commands = lines_from_file("data/day2_input.txt").unwrap();

    let newpos = commands
        .into_iter()
        .map(|line| line.parse::<SubCommand>().unwrap())
        .fold(SubPosition::default(), |p, comm| p + comm);

    println!("{:?}", newpos.depth * newpos.horizontal)
}

pub fn part_two() {
    let commands = lines_from_file("data/day2_input.txt").unwrap();

    let newpos = commands
        .into_iter()
        .map(|line| line.parse::<SubCommand>().unwrap())
        .fold(SubPositionActual::default(), |p, comm| p + comm);

    println!("{:?}", newpos.depth * newpos.horizontal)
}