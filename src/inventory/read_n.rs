use std::fs;
pub fn main(path: &str) -> Result<i32, ()> {
	let n = fs::read_to_string(&path)
		.ok()
		.and_then(|s| s.parse::<i32>().ok())
		.unwrap_or(0);
	Ok(n)
}
