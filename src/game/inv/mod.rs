mod store;
use {
	crate::{
		cleanse,
		game::inv::store::{
			InventoryStore,
			Item,
			get_item_path,
		},
		read_n,
	},
	std::path::PathBuf,
};
pub(super) fn add(mut item: String, increase: u8) {
	let max: u8 = 1 << 6;
	let path: PathBuf = get_item_path(&item);
	let mut count: u8 = read_n(&path);
	let old: u8 = count;
	count = (old + increase).min(max);
	if old == max {
		println!("This slot is full");
	} else if count == max {
		println!("This slot is now full");
	}
	item = cleanse(item);
	InventoryStore {}.set(&item, count);
	println!("{item}×{count}");
}
pub(super) fn check(item: String, target: u8) {
	let Item { id, count, path: _ } = InventoryStore {}.get(item);
	if count >= target {
		println!("You have {id}×{target} ({count})");
	} else {
		println!("You do not have {id}×{target} ({count})");
	}
}
pub(super) fn drop(item: String, decrease: u8) {
	let inv: InventoryStore = InventoryStore {};
	let Item {
		id,
		mut count,
		path: _,
	} = inv.get(item.clone());
	if count == 0_u8 {
		println!("You have nothing to drop");
	} else if count <= decrease {
		inv.remove(item);
		count = 0_u8;
	} else {
		count -= decrease;
		inv.set(&id, count);
	}
	println!("{id}×{count}");
}
pub(super) fn list() {
	let item_vec = InventoryStore {}.list();
	if item_vec.is_empty() {
		println!("Your inventory is empty...")
	} else {
		for (item, count) in item_vec {
			println!("{item}×{count}");
		}
	}
}
