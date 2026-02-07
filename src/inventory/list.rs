use {
	crate::utils::*,
	std::fs,
};
pub fn main() {
	let path = format!("{ROOT}/.state/items");
	let items = fs::read_dir(&path).unwrap();
	let mut item_count = 0;
	for i in items {
		let item = i.unwrap().path();
		let dir = format!("{}/", &path).to_string();
		println!(
			"{} x {}",
			item.display().to_string().replace(&dir, ""),
			fs::read_to_string(&item).unwrap().parse::<i32>().unwrap()
		);
		item_count += 1;
	}
	if item_count == 0 {
		println!("Your inventory is empty...")
	}
}
