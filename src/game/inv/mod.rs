use crate::{
	ROOT,
	game::inv::store::{
		InventoryStore,
		Item,
	},
};
mod store;
pub(super) fn add(item: String, increase: u8) {
	let inv = InventoryStore::new(ROOT);
	let max = 1_u8 << 6_u8;
	let path = inv.item_path(&item);
	let mut count = inv.read_n(&path);
	let old = count;
	count = (old + increase).min(max);
	if old == max {
		println!("This slot is full");
	} else if count == max {
		println!("This slot is now full");
	}
	inv.set(&item, count);
	println!("{item}×{count}");
}
pub(super) fn check(item: &str, target: u8) {
	let Item { id, count, path: _ } = InventoryStore::new(ROOT).get(item);
	if count >= target {
		println!("You have {id}×{target} ({count})");
	} else {
		println!("You do not have {id}×{target} ({count})");
	}
}
pub(super) fn drop(item: &str, decrease: u8) {
	let inv = InventoryStore::new(ROOT);
	let Item {
		id,
		mut count,
		path: _,
	} = inv.get(item);
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
	let item_vec = InventoryStore::new(ROOT).list();
	if item_vec.is_empty() {
		println!("Your inventory is empty...")
	} else {
		for (item, count) in item_vec {
			println!("{item}×{count}");
		}
	}
}
