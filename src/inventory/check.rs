use {
	crate::{
		ROOT,
		inventory::read_n::main as read_n,
	},
	std::result::Result,
};
pub fn main(item: &str, target: Option<i32>) -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items/{item}");
	let count = read_n(&path).expect("Invalid item count found");
	let tar: i32;
	if let Some(t) = target {
		tar = t
	} else {
		tar = 1
	}
	if count >= tar {
		println!("You have {item}×{tar} ({count})");
		Ok(true)
	} else {
		println!("You do not have {item}×{tar} ({count})");
		Ok(false)
	}
}
