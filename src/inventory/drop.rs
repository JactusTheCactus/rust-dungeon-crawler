use {
	crate::{
		inventory::read_n::main as read_n,
		utils::*,
	},
	std::fs,
};
pub fn main(item: &str, decrease: Option<i32>) -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path).expect("Invalid item count found");
	let mut decrement = 1;
	if let Some(d) = decrease {
		decrement = d
	}
	if count <= decrement {
		let _ = fs::remove_file(&path);
		count = 0;
	} else {
		count -= decrement;
		let _ = fs::write(&path, count.to_string());
	}
	println!("{item}×{count}");
	Ok(true)
}
