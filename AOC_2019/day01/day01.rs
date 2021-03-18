use std::fs::read_to_string;
use std::iter::successors;

fn read_a_file() -> Vec<u32> {
    read_to_string("input.txt")
        .expect("Unable to read input.txt")
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

fn calc_fuel(mass: &u32) -> Option<u32> {
    (mass / 3).checked_sub(2)
}

fn calc_full_fuel(mass: &u32) -> u32 {
    successors(Some(*mass), calc_fuel).skip(1).sum()
}

fn main() {
    let vec: Vec<u32> = read_a_file();

    let part1: u32 = vec.iter()
                        .map(|line| calc_fuel(line).unwrap())
                        .sum();
    
    let part2: u32 = vec.iter()
                        .map(calc_full_fuel)
                        .sum();

    println!("Part one answer: {}", part1);
    println!("Part two answer: {}", part2);
}