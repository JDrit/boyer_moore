
extern crate boyer_moore;

use std::env;
use std::process;
use std::fs::File;

use boyer_moore::search::search;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("not enough arguments, run {} [pattern] [input file]", args[0]);
        process::exit(1);
    }

    let ref pattern = args[1];
    let ref file_name =  args[2];
    let file: File = File::open(file_name).unwrap();

    search::search_file(pattern, file);
}
