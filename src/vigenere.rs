use std::char;

pub enum CipherMode {
	ENCODE,
	DECODE,
}

pub fn do_final(input: &String, key: &str, mode: &CipherMode) -> String {
	let key_chars: Vec<char> = key.chars().collect();
	let mut output = Vec::new();
	let mut i = 0;

	for input_char in input.chars() {
		if !input_char.is_alphabetic() {
			output.push(input_char);
			continue;
		}
		output.push(change_char(
			input_char,
			key_chars[i % key_chars.len()],
			mode,
		));
		i += 1;
	}

	output.into_iter().collect()
}

pub fn change_char(input: char, key_char: char, mode: &CipherMode) -> char {
	let (key_char_corrected, initial_char) = if input.is_lowercase() {
		(key_char.to_ascii_lowercase(), 'a')
	} else if input.is_uppercase() {
		(key_char.to_ascii_uppercase(), 'A')
	} else {
		panic!("character {} is neither lowercase or uppercase", input);
	};

	let mut result = ((input as u8) as i32
		+ ((match mode {
			CipherMode::ENCODE => 1,
			CipherMode::DECODE => -1,
		}) * ((key_char_corrected as u8) - (initial_char as u8)) as i32)) as u8;

	if input.is_ascii_lowercase() {
		result = match result as u8 {
			0...96 => result + 26,
			123...150 => result - 26,
			_ => result,
		}
	} else if input.is_ascii_uppercase() {
		result = match result as u8 {
			0...64 => result + 26,
			91...127 => result - 26,
			_ => result,
		}
	}

	result as char
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_encode() {
		let input = "ThIS is jUSt aN ExAmPLE".to_string();
		let key = "KeY";
		let result = do_final(&input, &key, &CipherMode::ENCODE);

		assert_eq!(result, "DlGC mq tYQd eL ObYwTJO");
	}

	#[test]
	fn test_decode() {
		let input = "DlGC mq tYQd eL ObYwTJO".to_string();
		let key = "kEy";
		let result = do_final(&input, &key, &CipherMode::DECODE);

		assert_eq!(result, "ThIS is jUSt aN ExAmPLE");
	}

	#[test]
	fn test_bigger_key_than_input_encode() {
		let input = "teSt".to_string();
		let key = "thisIsABIGKey";
		let result = do_final(&input, &key, &CipherMode::ENCODE);

		assert_eq!(result, "mlAl");
	}

	#[test]
	fn test_bigger_key_than_input_decode() {
		let input = "mlAl".to_string();
		let key = "thisIsABIGKey";
		let result = do_final(&input, &key, &CipherMode::DECODE);

		assert_eq!(result, "teSt");
	}
}