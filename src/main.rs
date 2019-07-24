#[macro_use]
extern crate clap;
use clap::App;
use std::fs::File;

use std::io::prelude::*;
use std::io::BufReader;
mod vigenere;

const INPUT: &str = "input";
const MODE: &str = "mode";
const ENCODE: &str = "encode";
const KEY: &str = "key";

fn main() {
	let yaml = load_yaml!("config/cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let key = matches.value_of(KEY).unwrap();

	if !key.chars().all(char::is_alphabetic) {
		panic!("Key is not alphabetical");
	}


	let mut string_read: String = String::new();

	if matches.is_present(INPUT) {
		read_input_file(INPUT, &mut string_read);
	} else {
		read_from_stdin(&mut string_read);
	}

	let mut string_result: String = String::new();

	//This argument is required so i don't need to check if it is present
	if matches.value_of(MODE).unwrap() == ENCODE {
		vigenere::encode(&string_read, &mut string_result, &key);
	}
	//Argument only has 2 possible values
	else {
		vigenere::decode(&string_read, &mut string_result, &key);
	}

}

fn read_input_file(input_file_name: &str, file_content: &mut String) {
	let file = File::open(input_file_name).expect("Error reading file!");
	let mut buf_reader = BufReader::new(file);
	buf_reader
		.read_to_string(file_content)
		.expect("Error reading file to a string!");
}

fn read_from_stdin(file_content: &mut String) {
	let mut stdin = std::io::stdin();
	stdin
		.read_to_string(file_content)
		.expect("Error reading from stdin!");
}
