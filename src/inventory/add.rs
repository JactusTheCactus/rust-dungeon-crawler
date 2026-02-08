use {
	crate::utils::{
		ROOT,
		read_n,
	},
	std::fs,
};
pub fn add(item: &str, increase: Option<i32>) -> bool {
	let max = 0b1000000;
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path);
	let mut increment = 1;
	if let Some(i) = increase {
		if i > 0 {
			increment = i
		} else {
			println!("Err: Increase must be a positive integer");
			return false;
		}
	}
	count += increment;
	if count >= max {
		count = max;
		println!("This slot is full")
	}
	let _ = fs::write(&path, count.to_string());
	println!("{item}×{count}");
	true
}
