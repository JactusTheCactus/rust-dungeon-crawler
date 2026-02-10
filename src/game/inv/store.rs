use {
	crate::{
		cleanse,
		read_n,
	},
	std::{
		fs::{
			read_dir,
			remove_file,
			write,
		},
		path::{
			Path,
			PathBuf,
		},
	},
};
pub(super) fn get_item_path(item: &str) -> PathBuf {
	Path::new(crate::ROOT)
		.join(".state")
		.join("items")
		.join(&crate::cleanse(item))
}
pub(super) struct Item {
	pub(super) id: String,
	pub(super) count: u8,
	pub(super) path: PathBuf,
}
pub(super) struct InventoryStore {
	pub(super) root: PathBuf,
}
impl InventoryStore {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}
	pub(super) fn get(&self, item: &str) -> Item {
		let path = get_item_path(&item);
		let count = read_n(&path);
		return Item {
			id: cleanse(item),
			count,
			path,
		};
	}
	pub(super) fn set(&self, item: &str, count: u8) {
		if let Err(e) = write(&get_item_path(&item), count.to_string()) {
			eprintln!("Failed to write to file: {e}");
		}
	}
	pub(super) fn remove(&self, item: &str) {
		let Item {
			id: _,
			count: _,
			path,
		} = self.get(item);
		if let Err(e) = remove_file(&path) {
			eprintln!("Failed to remove file: {e}")
		}
	}
	pub(super) fn list(&self) -> Vec<(String, u8)> {
		let path = Path::new(&self.root).join(".state").join("items");
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
}
