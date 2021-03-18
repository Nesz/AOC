#[path="../machine.rs"]
mod machine;
use machine::Machine;

fn main() {
    let codes = machine::read_codes("input.txt");


    let a1 = Machine::new(codes.clone(), vec![1]).run();
    println!("Part one answer: {}", a1.unwrap());


    let a2 = Machine::new(codes.clone(), vec![5]).run();
    println!("Part two answer: {}", a2.unwrap());
}