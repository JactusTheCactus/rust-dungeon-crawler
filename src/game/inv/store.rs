use std::{
	fs::{
		read_dir,
		read_to_string,
		remove_file,
		write,
	},
	path::{
		Path,
		PathBuf,
	},
};
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
	pub(super) fn item_path(&self, item: &str) -> PathBuf {
		Path::new(&self.root)
			.join(".state")
			.join("items")
			.join(&item)
	}
	pub(crate) fn read_n(&self, path: &Path) -> u8 {
		read_to_string(path)
			.ok()
			.and_then(|s| s.parse().ok())
			.unwrap_or(0)
	}
	pub(super) fn get(&self, item: &str) -> Item {
		let path = self.item_path(&item);
		let count = self.read_n(&path);
		Item {
			id: item.to_string(),
			count,
			path,
		}
	}
	pub(super) fn set(&self, item: &str, count: u8) {
		if let Err(e) = write(&self.item_path(&item), count.to_string()) {
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
						self.read_n(&entry.path()),
					));
				}
			}
			item_vec.sort_by(|(a, _), (b, _)| a.cmp(&b));
		}
		item_vec
	}
}
