pub mod cli;
pub mod game;
use {
	once_cell::sync::Lazy,
	regex::Regex,
	std::{
		fs::read_to_string,
		path::Path,
	},
};
pub const ROOT: &str = "dungeon";
pub(crate) fn read_n(path: &Path) -> u8 {
	read_to_string(path)
		.ok()
		.and_then(|s: String| s.parse().ok())
		.unwrap_or(0)
}
static CLEANSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[/.\s]").unwrap());
pub(crate) fn cleanse(input: String) -> String {
	CLEANSE_RE
		.replace_all(input.as_str(), "_")
		.to_string()
		.to_lowercase()
}
