mod search;

use std::env;
use std::process;
use std::fs::File;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("not enough arguments, run {} [pattern] [input file]", args[0]);
        process::exit(1);
    }

    let ref pattern = args[1];
    let ref file_name =  args[2];
    let file: File = File::open(file_name).unwrap();
    let result = search::search_file(pattern, file);

    if result.len() != 0 {
        println!("found in file");
    } else {
        println!("not found in file");
    }
}
