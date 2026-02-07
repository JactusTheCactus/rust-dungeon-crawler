use {
	crate::utils::*,
	std::fs,
};
pub fn main(args: &mut Vec<&str>, arg_len: usize) {
	if arg_len == 2 {
		if let [.., item] = args[0..3] {
			let path = format!("{ROOT}/.state/items/{item}");
			let content = fs::read_to_string(&path);
			let mut count: i32;
			if let Some(count_str) = content.ok() {
				count = count_str.parse::<i32>().unwrap();
			} else {
				count = 0;
			}
			count += 1;
			let _ = fs::write(&path, count.to_string());
			println!("{item} x {count}");
		}
	} else {
		println!(
			"'inventory {}' takes 1 argument (provided {}).",
			args[1],
			arg_len - 1
		)
	}
}
