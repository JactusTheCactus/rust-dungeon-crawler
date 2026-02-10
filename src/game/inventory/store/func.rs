use {
	crate::{
		ROOT,
		cleanse,
		game::inventory::store::{
			InventoryStore,
			Item,
			get_item_path,
		},
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
pub(super) fn get(mut item: String) -> Item {
	item = cleanse(item);
	let path = &Path::new(ROOT)
		.join(".state")
		.join("items")
		.join(&cleanse(item.to_string()));
	let count = read_n(path);
	return Item {
		id: item.to_string(),
		count: count,
		path: path.to_path_buf(),
	};
}
pub(super) fn set(item: &String, count: u8) {
	let path = get_item_path(&item);
	if let Err(e) = write(&path, count.to_string()) {
		eprintln!("Failed to write to file: {e}");
	}
}
pub(super) fn remove(item: String) {
	let Item {
		id: _,
		count: _,
		path,
	} = InventoryStore {}.get(item);
	if let Err(e) = remove_file(&path) {
		eprintln!("Failed to remove file: {e}")
	}
}
pub(super) fn list() -> Vec<(String, u8)> {
	let path = Path::new(ROOT).join(".state").join("items");
	let mut item_vec: Vec<(String, u8)> = Vec::new();
	if let Ok(items) = read_dir(&path) {
		for i in items {
			if let Ok(entry) = i {
				item_vec.push((
					entry.file_name().to_string_lossy().into_owned(),
					read_n(&entry.path()),
				));
			}
		}
		item_vec.sort_by(|(a, _), (b, _)| a.cmp(&b));
	}
	item_vec
}
