use {
	crate::utils::{
		MAX,
		ROOT,
		cleanse,
		read_n,
	},
	std::{
		fs::{
			read_dir,
			remove_file,
			write,
		},
		path::Path,
	},
};
pub fn add(mut item: String, increase: u8) {
	item = cleanse(item);
	let path = Path::new(ROOT).join(".state/items").join(&item);
	let mut count = read_n(&path.display().to_string());
	count += increase;
	if count >= MAX {
		if count == MAX {
			println!("This slot is full");
		} else {
			println!("This slot is now full");
		}
		count = MAX;
	}
	if let Err(e) = write(&path, count.to_string()) {
		eprintln!("Failed to write to file: {e}");
	}
	println!("{item}×{count}");
}
pub fn check(mut item: String, target: u8) {
	item = cleanse(item);
	let path = Path::new(ROOT).join(".state/items").join(&item);
	let count = read_n(&path.display().to_string());
	if count >= target {
		println!("You have {item}×{target} ({count})");
	} else {
		println!("You do not have {item}×{target} ({count})");
	}
}
pub fn drop(mut item: String, decrease: u8) {
	item = cleanse(item);
	let path = Path::new(ROOT).join(".state/items").join(&item);
	let mut count = read_n(&path.display().to_string());
	if count == 0 {
		println!("You have nothing to drop");
	} else if count <= decrease {
		if let Err(e) = remove_file(&path) {
			eprintln!("Failed to remove file: {e}")
		}
		count = 0;
	} else {
		count -= decrease;
		if let Err(e) = write(&path, count.to_string()) {
			eprintln!("Failed to write to file: {e}");
		}
	}
	println!("{item}×{count}");
}
pub fn list() {
	let path = Path::new(ROOT).join(".state/items");
	if let Ok(items) = read_dir(&path) {
		let mut item_vec: Vec<(String, u8)> = Vec::new();
		for i in items {
			if let Ok(entry) = i {
				let item = entry.path();
				let dir = format!("{}/", &path.display().to_string()).to_string();
				item_vec.push((
					item.display().to_string().replace(&dir, ""),
					read_n(&item.display().to_string()),
				));
			}
		}
		item_vec.sort_by(|a, b| a.0.cmp(&b.0));
		if item_vec.is_empty() {
			println!("Your inventory is empty...")
		} else {
			for (item, count) in item_vec {
				println!("{item}×{count}");
			}
		}
	}
}
