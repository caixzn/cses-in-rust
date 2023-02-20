use std::io;

fn main() {
    // read the number of elements
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    // create a boolean vector of size n
    // and set all elements to false
    let mut v: Vec<bool> = vec![false; n];

    // read the elements and set the corresponding element in the vector to true
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    for i in input.split_whitespace() {
        v[i.parse::<usize>().unwrap() - 1] = true;
    }

    // iterate over the vector and find the first false element
    for i in 0..n {
        if v[i] == false {
            println!("{}", i+1);
            break;
        }
    }
}