use {
	crate::utils::*,
	std::{
		collections::{
			BTreeSet,
			HashMap,
		},
		fs,
	},
};
pub fn list() -> bool {
	let path = format!("{ROOT}/.state/items");
	let items = fs::read_dir(&path).unwrap();
	let mut item_map: HashMap<String, i32> = HashMap::new();
	for i in items {
		let item = i.unwrap().path();
		let dir = format!("{}/", &path).to_string();
		item_map.insert(
			item.display().to_string().replace(&dir, ""),
			fs::read_to_string(&item).unwrap().parse::<i32>().unwrap(),
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
