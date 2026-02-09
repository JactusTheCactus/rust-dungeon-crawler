//! Project-wide utilities
use std::fs;
/// The root of the game save
pub const ROOT: &str = "dungeon";
/// Read the contents of a file as a 32-bit unsigned integer
pub fn read_n(path: &str) -> u32 {
	if let Some(str) = fs::read_to_string(&path).ok() {
		if let Some(n) = str.parse::<u32>().ok() {
			return n;
		}
	}
	0
}
