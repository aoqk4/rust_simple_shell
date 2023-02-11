use std::{io::stdin, process::Command};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let com = input.trim();

    Command::new(com).spawn().unwrap();
}
