
const MIN: u32 = 234208;
const MAX: u32 = 765869;

fn to_digits(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn is_sorted(v: &Vec<u32>) -> bool {
    (0..5).all(|i| v[i] <= v[i + 1])
}

fn has_two_exact(v: &Vec<u32>) -> bool {
    (0..5).any(|i| v[i] == v[i + 1])
}

fn has_two_exact_only(v: &Vec<u32>) -> bool {
    (0..5).any(|i| v.iter().filter(|&n| *n == v[i]).count() == 2)
}

fn main() {
    let mut pass1 = 0;
    let mut pass2 = 0;
    
    for i in MIN..MAX {
        let digits = to_digits(i);
        
        if is_sorted(&digits) {
            
            if has_two_exact(&digits) {
                pass1 += 1;
            }
            
            if has_two_exact_only(&digits) {
                pass2 += 1;
            }
        }
    }

    println!("Part one answer: {}", pass1);
    println!("Part two answer: {}", pass2);
}