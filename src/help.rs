use std::collections::HashMap;
pub fn main() {
	let i = "i[nventory]";
	let mut help: HashMap<String, HashMap<String, String>> = HashMap::new();
	help.insert(
		format!("inventory"),
		HashMap::new() as HashMap<String, String>,
	);
	let mut push = |obj, k, v: &str| {
		help.get_mut(obj).unwrap().insert(k, v.to_string());
	};
	push(
		"inventory",
		i.to_string(),
		"list all items in to your inventory",
	);
	push(
		"inventory",
		format!("{i} a[dd] <item>"),
		"add <item> to your inventory",
	);
	push(
		"inventory",
		format!("{i} d[rop] <item>"),
		"remove <item> from your inventory",
	);
	println!("Commands:");
	println!();
	println!("{i} [action] [arg]:");
	for (k, v) in help.get("inventory").unwrap() {
		println!("\t- {k}:");
		println!("\t\t{v}");
	}
}
