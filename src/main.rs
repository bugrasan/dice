use std::env;
// use std::fs;
use std::fs::File;
// use std::io::Read;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
// use rand::prelude::*;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename)
                    .expect("ERROR: unable to open.");
    let reader = BufReader::new(file);

    let content: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();
    let len = content.len();

    let dice = rand::thread_rng().gen_range(0..len);
    println!("{}", content[dice]);
}
