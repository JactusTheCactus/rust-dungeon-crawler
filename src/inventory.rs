//! Functions for inventory commands
use {
	crate::utils::{
		ROOT,
		read_n,
	},
	std::{
		collections::{
			BTreeSet,
			HashMap,
		},
		fs,
	},
};
/// Add items to the player's inventory,
/// with a default amount of 1
pub fn add(item: &str, increase: Option<u32>) -> bool {
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
/// Check if the player has an item,
/// with an optional target amount,
/// with a default value of 1
pub fn check(item: &str, target: Option<u32>) -> bool {
	let path = format!("{ROOT}/.state/items/{item}");
	let count = read_n(&path);
	let tar: u32;
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
/// Remove items from the player's inventory,
/// with a default amount of 1
pub fn drop(item: &str, decrease: Option<u32>) -> bool {
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
/// List the items in the player's inventory
pub fn list() -> bool {
	let path = format!("{ROOT}/.state/items");
	let items = fs::read_dir(&path).unwrap();
	let mut item_map: HashMap<String, u32> = HashMap::new();
	for i in items {
		let item = i.unwrap().path();
		let dir = format!("{}/", &path).to_string();
		item_map.insert(
			item.display().to_string().replace(&dir, ""),
			read_n(&item.display().to_string()),
		);
	}
	let item_set: BTreeSet<&String> = item_map.keys().collect();
	if item_map.len() == 0 {
		println!("Your inventory is empty...")
	} else {
		for item in item_set {
			if let Some(count) = item_map.get(item) {
				println!("{item}×{count}");
			}
		}
	}
	true
}
