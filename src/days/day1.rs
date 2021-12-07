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
    let depths: Vec<u32> = input.into_iter().map(|line| line.parse::<u32>().unwrap()).collect();
    let depth_increases = depths
        .windows(2)
        .filter(|depth| depth[1] > depth[0])
        .count();
    println!("Depth changes: {}", depth_increases)
}

fn part_two() {
    let input = lines_from_file("data/day1_input.txt").unwrap();
    let mut depths: Vec<u32> = input.into_iter().map(|line| line.parse::<u32>().unwrap()).collect();
    depths.extend_from_within(..2);

    let tri_depths =  depths
        .windows(3)
        .map(|tri| tri.into_iter().sum())
        .collect::<Vec<u32>>();

    let depth_increases = tri_depths
        .windows(2)
        .filter(|depth| depth[1] > depth[0])
        .count(); 
        
    
    println!("Depth changes: {}", depth_increases)
}