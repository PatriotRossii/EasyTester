use clap::{Arg, App};

use std::fs::{File};
use std::io::Read;

use tester_rs::get_real_output;

fn main() {
    let matches = App::new("Test system")
        .version("1.0")
        .author("Danil S. <patriotrossii2019@mail.ru>")
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("INPUT_FILE")
            .help("Sets the input file to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("OUTPUT_FILE")
            .help("Sets the output file to use")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("exec")
            .help("Sets the executable file to test")
            .required(true)
            .value_name("EXECUTABLE")
            .index(1))
        .get_matches();
    
    let mut input = String::new();
    let mut input_file = File::open(matches.value_of("input").unwrap())
        .expect("failed to open input file");
    input_file.read_to_string(&mut input).expect("failed to read input file");

    let mut required_output = String::new();
    let mut output_file = File::open(matches.value_of("output").unwrap())
        .expect("failed to open output file");
    output_file.read_to_string(&mut required_output).expect("failed to read output file");

    let real_output = get_real_output(matches.value_of("exec").unwrap(), &input);

    if real_output.trim() == required_output.trim() {
        println!("TEST HAS PASSED");
    } else {
        println!("TEST HAS FAILED");
    }
}