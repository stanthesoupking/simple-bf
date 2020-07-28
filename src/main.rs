use std::env;
use std::fs::File;
use std::io::prelude::*;

mod simple_bf;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_filename = args.get(1).expect("No input filename provided.");

    let mut input_file = File::open(input_filename)
        .expect(format!("Failed opening input file '{}'.", input_filename).as_str());

    // Read input file
    let mut contents = vec![];
    input_file
        .read_to_end(&mut contents)
        .expect("Failed reading input file.");

    let program_source = String::from_utf8(contents).unwrap();

    // Initialise brainfuck machine
    let mut bf_machine = simple_bf::BrainfuckMachine::new();
    bf_machine.load_program(&program_source);
    println!("-- Program starting.");
    bf_machine.run();
    println!("-- Program finished.");
}
