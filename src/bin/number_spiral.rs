use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<u64>().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        let pos: Vec<u64> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        println!("{}", get_spiral_value(&pos[0], &pos[1]));
    }
}

fn get_spiral_value(line: &u64, column: &u64) -> u64 {
    if line > column && line%2 == 0 {
        line*line - column + 1
    } else if line > column {
        (line - 1)*(line - 1) + column
    } else if column%2 == 0 {
        (column - 1)*(column - 1) + line
    } else {
        column*column - line + 1
    }
}