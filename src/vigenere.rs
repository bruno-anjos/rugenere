use std::char;

pub fn encode(input: &String, key: &str) -> String{
	let key_chars: Vec<char> = key.chars().collect();
	let output = vec![0; input.len()];

	for (i, input_char) in input.chars().enumerate() {
		output[i] = if input_char.is_uppercase() {
			(input_char as i32) + ((key_chars[i % key_chars.len()] as i32) - ('a' as i32))
		} else {
			
		}
	}

	output.into_iter().collect();
}

pub fn decode(input: &String, output: &mut String, key: &str) {}
