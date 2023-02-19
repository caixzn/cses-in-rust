use std::io;

fn main() {
    // read a string
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // find the maximum length of a substring with repeated characters
    let mut max: u64 = 0;
    let mut count: u64 = 1;
    for i in 1..input.len() {
        if input.as_bytes()[i] == input.as_bytes()[i-1] {
            count += 1;
        } else {
            if count > max {
                max = count;
            }
            count = 1;
        }
    }

    // print the result
    println!("{}", max);
}
