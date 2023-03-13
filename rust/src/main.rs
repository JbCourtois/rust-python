use std::io::{self, BufRead};

// use postflop_solver::*;
// use std::convert::TryFrom;

fn main() {
    eprintln!("Ready");

    loop {
        let num = read_int();
        println!("{}", num * 2);
    }
}

fn read_int() -> i32 {
    let stdin = io::stdin();
    let mut line = String::new();

    loop {
        stdin.lock().read_line(&mut line).unwrap();

        let input = line.trim();
        if let Ok(num) = input.parse::<i32>() {
            break num;
        }
        eprintln!("Error: invalid input");
        line.clear();
    }
}
