use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n = input.trim().parse::<u64>().unwrap();

    // collatz conjecture
    while n != 1 {
        print!("{n} ");
        if n % 2 == 0 {
            n = n / 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("{}", n);
}
