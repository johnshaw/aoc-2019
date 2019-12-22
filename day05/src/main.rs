use std::io::*;
use common::*;

fn main() {
    let line = stdin().lock().lines().next().unwrap().unwrap();

    let output = intcode::run(&line, &[1]);
    println!("{:?}", output);

    let output = intcode::run(&line, &[5]);
    println!("{:?}", output);
}
