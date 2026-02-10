mod func;
use {
	crate::{
		ROOT,
		cleanse,
		game::inv::store::func::{
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
	Path::new(ROOT)
		.join(".state")
		.join("items")
		.join(&cleanse(item.to_string()))
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
