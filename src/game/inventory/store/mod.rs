mod func;
use {
	crate::{
		ROOT,
		game::inventory::store::func::{
			get,
			list,
			remove,
			set,
		},
	},
	std::path::{
		Path,
		PathBuf,
	},
};
pub(super) fn get_item_path(item: &String) -> PathBuf {
	Path::new(ROOT).join(".state/items").join(&item)
}
pub(super) struct Item {
	pub(super) id: String,
	pub(super) count: u8,
	pub(super) path: PathBuf,
}
pub(super) struct InventoryStore {}
impl InventoryStore {
	pub(super) fn get(&self, item: String) -> Item {
		get(item)
	}
	pub(super) fn set(&self, item: &String, count: u8) {
		set(item, count)
	}
	pub(super) fn remove(&self, item: String) {
		remove(item)
	}
	pub(super) fn list(&self) -> Vec<(String, u8)> {
		list()
	}
}
