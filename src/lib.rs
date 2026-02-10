pub mod cli;
pub mod game;
use {
	regex::Regex,
	std::fs::read_to_string,
};
pub const ROOT: &str = "dungeon";
pub const MAX: u8 = 1_u8 << 6_u8;
pub fn read_n(path: &str) -> u8 {
	if let Some(str) = read_to_string(&path).ok() {
		if let Some(n) = str.parse::<u8>().ok() {
			return n;
		}
	}
	0_u8
}
pub fn cleanse(input: String) -> String {
	Regex::new(r"[/.\s]")
		.unwrap()
		.replace_all(input.as_str(), "_")
		.to_string()
		.to_lowercase()
}
