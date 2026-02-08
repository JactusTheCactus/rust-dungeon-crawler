use crate::{
	ROOT,
	utils::read_n,
};
pub fn check(item: &str, target: Option<i32>) -> bool {
	let path = format!("{ROOT}/.state/items/{item}");
	let count = read_n(&path);
	let tar: i32;
	if let Some(t) = target {
		if t > 0 {
			tar = t
		} else {
			println!("Err: Target must be a positive integer");
			return false;
		}
	} else {
		tar = 1
	}
	if count >= tar {
		println!("You have {item}×{tar} ({count})");
		true
	} else {
		println!("You do not have {item}×{tar} ({count})");
		false
	}
}
