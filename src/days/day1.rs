use crate::common::lines_from_file;

pub fn run(part: u32) {
    match part {
        1 => part_one(),
        2 => part_two(),
        _ => eprintln!("That part isn't available!")
    }
}

fn part_one() {
    let input = lines_from_file("data/day1_input.txt").unwrap();
    let depths: Vec<u32> = input.iter().map(|line| line.parse::<u32>().unwrap()).collect();
    let depth_increases = depths
        .windows(2)
        .filter(|depth| depth[1] > depth[0])
        .count();
    println!("Depth changes: {}", depth_increases)
}

fn part_two() {
    todo!()
}