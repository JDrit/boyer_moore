mod search;

use std::env;
use std::process;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("not enough arguments");
        process::exit(1);
    }

    let ref pattern = args[1];
    let ref file_name =  args[2];
    let file = File::open(file_name);

    match file {
        Ok(file) => {
            let result = search::contains(pattern, file);
            if result {
                println!("found in file");
            } else {
                println!("not found in file");
            }
        },
        Err(e) => {
            println!("error found: {}", e);
        },
    }
}
