#[path="../machine.rs"]
mod machine;
use machine::Machine;

const MAGIC_NUMBER: i128 = 19690720;

fn bf(mut codes: Vec<i128>) -> i128 {
    for i in 0..100 {
        for j in 0..100 {
            codes[1] = i;
            codes[2] = j;
            let mut machine = Machine::new(codes.clone(), vec![1]);
            machine.run();
            if machine.memory[0] == MAGIC_NUMBER {
                return 100 * i + j;
            }
        }
    }
    panic!("Part two answer couldn't be found");
}

fn main() {
    let mut codes = machine::read_codes("input.txt");
    codes[1] = 12;
    codes[2] = 2;
    let mut machine = Machine::new(codes.clone(), vec![0]);
    machine.run();
    println!("Part one answer: {}", machine.memory[0]);
    println!("Part two answer: {}", bf(codes.clone()));

}