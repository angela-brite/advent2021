use crate::common::lines_from_file;

fn part_one() {
    let lines = lines_from_file("data/day3_input.txt").unwrap();
    
    let bit_lines = lines
        .into_iter()
        .map(|s| s
            .split("")
            .map(|c| c.parse::<u16>().unwrap())
            .collect::<Vec<u16>>()
        );
    
}
