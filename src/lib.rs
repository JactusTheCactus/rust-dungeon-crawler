use {
	once_cell::sync::Lazy,
	regex::Regex,
};
pub mod cli;
pub mod game;
pub const ROOT: &str = "dungeon";
static CLEANSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\w-]").unwrap());
pub(crate) fn cleanse(input: &str) -> String {
	CLEANSE_RE.replace_all(input, "_").to_lowercase()
}
