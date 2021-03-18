use std::collections::HashMap;
use std::fs::read_to_string;

pub fn orbit_parser(file: &str) ->  HashMap<String, String> {
    read_to_string(file)
        .expect("Unable to read file")
        .trim()
        .lines()
        .map(|kv| kv.split(')').collect::<Vec<&str>>())
        .map(|vec| (vec[1].to_string(), vec[0].to_string()))
        .collect()
}

fn orbits_of(map: &HashMap<String, String>, object: &String) -> usize {
    match map.get(object) {
        None => 0,
        Some(center) => 1 + orbits_of(&map, center),
    }
}


fn main() {
    let mut v = orbit_parser("input.txt");
    let mut sum = 0;
    for (k, z) in v.iter() {
        sum += orbits_of(&v, k)
    }
    println!("{}", sum);
}