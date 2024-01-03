use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("echo howdy!");
}
