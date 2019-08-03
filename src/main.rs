#[macro_use]
extern crate clap;
use clap::App;
use std::fs;
use std::fs::File;

use std::io::Write;

use std::io::Result;
mod vigenere;

const INPUT: &str = "input";
const OUTPUT: &str = "output";
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
		string_read = read_input_file(matches.value_of(INPUT).unwrap()).unwrap();
	} else {
		read_from_stdin(&mut string_read);
		string_read = string_read.trim().to_string();
	}

	let mut string_result;

	//This argument is required, so i don't need to check if it is present
	if matches.value_of(MODE).unwrap() == ENCODE {
		string_result = vigenere::do_final(&string_read, &key, &vigenere::CipherMode::ENCODE);
	}
	//Argument only has 2 possible values
	else {
		string_result = vigenere::do_final(&string_read, &key, &vigenere::CipherMode::DECODE);
	}

	if matches.is_present(OUTPUT) {
		write_to_file(matches.value_of(OUTPUT).unwrap(), &string_result);
	} else {
		println!("Result: {}", string_result);
	}

}

fn read_input_file(input_file_name: &str) -> Result<String> {
	let content = fs::read_to_string(input_file_name)?;
	Ok(content.trim().to_string())
}

fn read_from_stdin(file_content: &mut String) {
	let stdin = std::io::stdin();
	stdin
		.read_line(file_content)
		.expect("Error reading from stdin!");
}

fn write_to_file(file_name: &str, output: &str) {
	let mut file = File::create(file_name).unwrap();
	file.write_all(output.as_bytes())
		.expect("Error writing to file!");
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_input_file() {
		let input = read_input_file("input_test").unwrap();
		assert_eq!(input, "ThIS is jUSt aN ExAMple");
	}

	#[test]
	fn test_output_file() {
		let output = "DlGC mq tYQd eL ObYWtjo";
		write_to_file("output_test", &output);
		assert_eq!(read_input_file("output_test").unwrap(), output);
	}
}
