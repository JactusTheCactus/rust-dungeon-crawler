use std::fs;
pub const ROOT: &str = "dungeon";
pub fn read_n(path: &str) -> i32 {
	if let Some(str) = fs::read_to_string(&path).ok() {
		if let Some(n) = str.parse::<i32>().ok() {
			return n;
		}
	}
	0
}
