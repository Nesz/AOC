#[path="../machine.rs"]
mod machine;
use machine::Machine;
use machine::Status;

fn main() {
    let codes = machine::read_codes("input.txt");

    println!("{}", find(&codes, 0, 4));
    println!("{}", find(&codes, 5, 9));
}

pub fn signal_output(codes: &Vec<i128>, seq: &Vec<i128>) -> i128 {
    let mut a = Machine::new(codes.clone(), vec![seq[0], 0]);
    let mut b = Machine::new(codes.clone(), vec![seq[1]]);
    let mut c = Machine::new(codes.clone(), vec![seq[2]]);
    let mut d = Machine::new(codes.clone(), vec![seq[3]]);
    let mut e = Machine::new(codes.clone(), vec![seq[4]]);


    let mut signal = 0;

    'outer: loop {
        let val_a = loop {
            a.tick();
            if let Status::Halted = a.status {
                break 'outer;
            }
            if a.output.len() > 0 {
                break a.output.remove(0);
            }
        };

        b.input.push(val_a);

        let val_b = loop {
            b.tick();
            if let Status::Halted = b.status {
                break 'outer;
            }
            if b.output.len() > 0 {
                break b.output.remove(0);
            }
        };


        c.input.push(val_b);

        let val_c = loop {
            c.tick();
            if let Status::Halted = c.status {
                break 'outer;
            }
            if c.output.len() > 0 {
                break c.output.remove(0);
            }
        };

        d.input.push(val_c);

        let val_d = loop {
            d.tick();
            if let Status::Halted = d.status {
                break 'outer;
            }
            if d.output.len() > 0 {
                break d.output.remove(0);
            }
        };

        e.input.push(val_d);

        let val_e = loop {
            e.tick();
            if let Status::Halted = e.status {
                break 'outer;
            }
            if e.output.len() > 0 {
                break e.output.remove(0);
            }
        };

        signal = val_e;
        a.input.push(val_e);
    };
    
    signal
}

pub fn find(codes: &Vec<i128>, from: i128, to: i128) -> i128 {
    let mut highest = 0;

    for a in from..=to {

        for b in from..=to {
            
            if b == a { 
                continue; 
            }

            for c in from..=to {

                if c == b || c == a { 
                    continue; 
                }

                for d in from..=to {
                    if d == c || d == b || d == a { 
                        continue; 
                    }

                    for e in from..=to {
                        if e == d || e == c || e == b || e == a { 
                            continue; 
                        }

                        let seq = vec![a, b, c, d, e];
                        let out = signal_output(&codes, &seq);
                        
                        if out > highest {
                            highest = out;
                        }
                    }
                }
            }
        }
    }

    highest
}