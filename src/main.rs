use bcrypt::{hash, verify};

use std::env;
use std::io::{self, Write};

fn main() {
    // Get command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        io::stderr()
            .write_all("Must define the cost".as_bytes())
            .unwrap();
        std::process::exit(1)
    }

    let cost: u32 = args[1].parse::<u32>().unwrap();

    if (cost as i32) < 4 || (cost as i32) > 31 {
        io::stderr()
            .write_all("Cost must be between 4..31".as_bytes())
            .unwrap();
        std::process::exit(1)
    }

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let hashed = hash(buffer.clone(), cost).unwrap();

    let valid = verify(buffer.clone(), &hashed).unwrap();

    if valid {
        io::stdout().write_all(hashed.as_bytes()).unwrap();
    } else {
        io::stderr()
            .write_all("Not a valid password".as_bytes())
            .unwrap();
    }
}
