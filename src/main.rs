mod utils;
use utils::*;

use std::env;

// if argument == cd => write state
// else -> "cd read_state"
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        1 => {
            manage_file::write_state(&args[0]).unwrap();
        }
        _ => {
            println!("{}", hook::redefine_cd());
            let path = manage_file::read_state().unwrap();
            println!("cd {}", path);
        }
    }
}
