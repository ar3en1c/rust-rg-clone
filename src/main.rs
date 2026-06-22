
mod search;


use std::collections::HashMap;
use std::env::args;
use std::{fs, usize};
use std::io::Read;

fn main() {
    //colored::control::set_override(true);
    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        println!("Usage: {} <words> <file>", args[0]);
        return;
    }
    let filename = &args[2];
    let mut file = match fs::File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let mut fileread: String = String::new();
    match file.read_to_string(&mut fileread) {
        Ok(f) => f,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let vetorize: Vec<&str> = fileread.lines().collect();
    let my: HashMap<String, Vec<usize>> = search::search(&args[1], &vetorize);

    for u in &my[&args[1]]{
        //println!("{}", vetorize[*u - 1]);
        let print = search::sentence(vetorize[*u - 1], &args[1]);
        println!("{}: {}", &u ,print);
    }

    

    //println!("{:#?}", vetorize);
}
