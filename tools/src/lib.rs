pub mod parser {
	pub fn string(day: &str) -> String {
		let mut s = String::from("./input/");
		s.push_str(day);
		s.push_str(".input");
		match std::fs::read_to_string(s) {
			Ok(file) => file,
			Err(error) => panic!("Error occured while reading file: {}", error)
		}
	}
	pub fn number_array(day: &str, sep: &str) -> Vec<i32> {
		string(day).split(sep).map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
	}
}