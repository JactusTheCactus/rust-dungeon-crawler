use {
	crate::{
		inventory::read_n::main as read_n,
		utils::*,
	},
	std::fs,
};
pub fn main(args: &mut Vec<&str>) {
	if args.len() == 3 {
		if let Some(item) = args.get(2) {
			let path = format!("{ROOT}/.state/items/{item}");
			let mut count: i32 = read_n(&path);
			count += 1;
			let _ = fs::write(&path, count.to_string());
			println!("{item} x {count}");
		}
	} else {
		println!(
			"'inventory {}' takes 1 argument (provided {}).",
			args[1],
			args.len() - 2
		)
	}
}
