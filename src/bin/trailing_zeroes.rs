use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<u64>().unwrap();

    let mut count = 0;
    let mut d = 5;
    while n / d > 0 {
        count += n / d;
        d *= 5;
    }

    println!("{}", count);
}