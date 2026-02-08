use {
	crate::utils::{
		read_n,
		*,
	},
	std::fs,
};
pub fn drop(item: &str, decrease: Option<i32>) -> bool {
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path);
	let mut decrement = 1;
	if let Some(d) = decrease {
		if d > 0 {
			decrement = d
		} else {
			println!("Err: Decrease must be a positive integer");
			return false;
		}
	}
	if count <= decrement {
		let _ = fs::remove_file(&path);
		count = 0;
	} else {
		count -= decrement;
		let _ = fs::write(&path, count.to_string());
	}
	println!("{item}×{count}");
	true
}
