use {
	crate::{
		inventory::read_n::main as read_n,
		utils::*,
	},
	std::fs,
};
pub fn main(item: &str, increase: Option<i32>) -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path).expect("Invalid item count found");
	let mut increment = 1;
	if let Some(i) = increase {
		increment = i
	}
	count += increment;
	let _ = fs::write(&path, count.to_string());
	println!("{item}×{count}");
	Ok(true)
}
