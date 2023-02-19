use std::io;

fn main() {
    // ignore first line of input
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // read the array
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut v: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // count the number of moves
    let mut moves: u64 = 0;
    for i in 1..v.len() {
        if v[i] < v[i-1] {
            moves += v[i-1] - v[i];
            v[i] = v[i-1];
        }
    }

    // print the result
    println!("{}", moves);
}