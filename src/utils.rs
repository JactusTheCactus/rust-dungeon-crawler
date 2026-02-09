use std::fs::read_to_string;
pub const ROOT: &str = "dungeon";
pub const MAX: u8 = 64;
pub fn read_n(path: &str) -> u8 {
	if let Some(str) = read_to_string(&path).ok() {
		if let Some(n) = str.parse::<u8>().ok() {
			return n;
		}
	}
	0
}
pub fn cleanse(input: String) -> String {
	input.replace("/", "_").replace(".", "_")
}
