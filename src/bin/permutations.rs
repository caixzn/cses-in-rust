use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();

    if n > 1 && n < 4 {
        println!("NO SOLUTION");
    } else {
        for i in (2..=n).step_by(2) {
            print!("{} ", i);
        }
        for i in (1..=n).step_by(2) {
            print!("{} ", i);
        }
        println!();
    }
}