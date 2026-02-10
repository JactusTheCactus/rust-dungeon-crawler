use {
	once_cell::sync::Lazy,
	regex::Regex,
	std::path::Path,
};
pub mod cli;
pub mod game;
pub const ROOT: &str = "dungeon";
pub(crate) fn read_n(path: &Path) -> u8 {
	std::fs::read_to_string(path)
		.ok()
		.and_then(|s| s.parse().ok())
		.unwrap_or(0)
}
static CLEANSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[/.\s]").unwrap());
pub(crate) fn cleanse(input: &str) -> String {
	CLEANSE_RE.replace_all(input, "_").to_lowercase()
}
