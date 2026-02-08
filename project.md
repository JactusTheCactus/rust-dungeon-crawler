Thoughts?
# `help.rs`
```rs
struct Command {
	cmd: String,
	text: String,
}
fn opt(r#type: &str) -> String {
	format!("with an optional {type} amount (defaults to 1 if not specified)")
}
pub fn main() -> Result<bool, ()> {
	let i = "i[nventory]";
	let help = &[
		Command {
			cmd: format!("{i} [l[ist]]"),
			text: format!("List all items in to your inventory"),
		},
		Command {
			cmd: format!("{i} a[dd] <item> [increase]"),
			text: format!("Add <item> to your inventory, {}", opt("increase")),
		},
		Command {
			cmd: format!("{i} c[heck] <item> [target]"),
			text: format!("Check if <item> is in your inventory, {}", opt("target")),
		},
		Command {
			cmd: format!("{i} d[rop] <item> [decrease]"),
			text: format!("Remove <item> from your inventory, {}", opt("decrease")),
		},
	];
	println!("Commands:");
	for Command { cmd, text } in help {
		println!("\t{cmd}\n\t\t{text}");
	}
	Ok(true)
}
```
# `inventory/add.rs`
```rs
use {
	crate::{
		inventory::read_n::main as read_n,
		utils::*,
	},
	std::fs,
};
pub fn main(item: &str, increase: Option<i32>) -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path).expect("Invalid item count found");
	let mut increment = 1;
	if let Some(i) = increase {
		increment = i
	}
	count += increment;
	let _ = fs::write(&path, count.to_string());
	println!("{item}×{count}");
	Ok(true)
}
```
# `inventory/check.rs`
```rs
use {
	crate::{
		ROOT,
		inventory::read_n::main as read_n,
	},
	std::result::Result,
};
pub fn main(item: &str, target: Option<i32>) -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items/{item}");
	let count = read_n(&path).expect("Invalid item count found");
	let tar: i32;
	if let Some(t) = target {
		tar = t
	} else {
		tar = 1
	}
	if count >= tar {
		println!("You have {item}×{tar} ({count})");
		Ok(true)
	} else {
		println!("You do not have {item}×{tar} ({count})");
		Ok(false)
	}
}
```
# `inventory/drop.rs`
```rs
use {
	crate::{
		inventory::read_n::main as read_n,
		utils::*,
	},
	std::fs,
};
pub fn main(item: &str, decrease: Option<i32>) -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path).expect("Invalid item count found");
	let mut decrement = 1;
	if let Some(d) = decrease {
		decrement = d
	}
	if count <= decrement {
		let _ = fs::remove_file(&path);
		count = 0;
	} else {
		count -= decrement;
		let _ = fs::write(&path, count.to_string());
	}
	println!("{item}×{count}");
	Ok(true)
}
```
# `inventory/list.rs`
```rs
use {
	crate::utils::*,
	std::{
		collections::{
			BTreeSet,
			HashMap,
		},
		fs,
	},
};
pub fn main() -> Result<bool, ()> {
	let path = format!("{ROOT}/.state/items");
	let items = fs::read_dir(&path).unwrap();
	let mut item_map: HashMap<String, i32> = HashMap::new();
	for i in items {
		let item = i.unwrap().path();
		let dir = format!("{}/", &path).to_string();
		item_map.insert(
			item.display().to_string().replace(&dir, ""),
			fs::read_to_string(&item).unwrap().parse::<i32>().unwrap(),
		);
	}
	let item_set: BTreeSet<&String> = item_map.keys().collect();
	if item_map.len() == 0 {
		println!("Your inventory is empty...")
	} else {
		for item in item_set {
			if let Some(count) = item_map.get(item) {
				println!("{item}×{count}");
			}
		}
	}
	Ok(true)
}
```
# `inventory/mod.rs`
```rs
mod add;
mod check;
mod drop;
mod list;
mod read_n;
use {
	add::main as add,
	check::main as check,
	drop::main as drop,
	list::main as list,
};
pub fn run(args: &mut Vec<&str>) -> Result<bool, ()> {
	if let Some(cmd) = args.get(1) {
		let num_args = args.len() - 2;
		match *cmd {
			| "a" | "add" => {
				if let Some(item) = args.get(2) {
					if let Some(i) = args.get(3) {
						add(item, Some(i.parse::<i32>().unwrap())).expect("Invalid inventory add");
					} else {
						add(item, None).expect("Invalid inventory add");
					}
				} else {
					println!("'inventory add' takes 1-2 arguments (provided {num_args}).")
				}
			}
			| "d" | "drop" => {
				if let Some(item) = args.get(2) {
					if let Some(i) = args.get(3) {
						drop(item, Some(i.parse::<i32>().unwrap()))
							.expect("Invalid inventory drop");
					} else {
						drop(item, None).expect("Invalid inventory drop");
					}
				} else {
					println!("'inventory drop' takes 1-2 arguments (provided {num_args}).")
				}
			}
			| "c" | "check" => {
				if let Some(i) = args.get(2) {
					if let Some(t) = args.get(3) {
						let target = t.parse::<i32>().unwrap();
						check(i, Some(target)).expect("Invalid inventory check");
					} else {
						check(i, None).expect("Invalid inventory check");
					}
				} else {
					println!("'inventory check' takes 1-2 arguments (provided {num_args}).")
				}
			}
			| "l" | "list" => {
				list().expect("Invalid inventory list");
			}
			| _ => println!("'{cmd}' is an invalid argument. Try again."),
		}
	} else {
		list().expect("Invalid inventory list");
	}
	Ok(true)
}
```
# `inventory/read_n.rs`
```rs
use std::fs;
pub fn main(path: &str) -> Result<i32, ()> {
	let n = fs::read_to_string(&path)
		.ok()
		.and_then(|s| s.parse::<i32>().ok())
		.unwrap_or(0);
	Ok(n)
}
```
# `main.rs`
```rs
mod help;
mod inventory;
mod utils;
use {
	clearscreen::clear,
	help::main as help,
	std::{
		fs,
		io::Write,
	},
	utils::*,
};
fn make_dirs() -> std::io::Result<()> {
	for dir in [".state/items"] {
		let _ = fs::create_dir_all(format!("{ROOT}/{dir}"));
	}
	Ok(())
}
fn main() {
	if make_dirs().is_ok() {
		println!("Welcome to the dungeon!");
		loop {
			let mut input = String::new();
			print!("> ");
			let _ = std::io::stdout().flush();
			std::io::stdin()
				.read_line(&mut input)
				.expect("That is an invalid command. Try again.");
			clear().expect("Failed to clear screen...");
			let mut cmd = input.trim().split_whitespace().collect::<Vec<&str>>();
			if let Some(command) = cmd.get(0) {
				match *command {
					| "i" | "inventory" => {
						let _ = inventory::run(&mut cmd);
					}
					| "q" | "quit" => {
						break;
					}
					| _ => {
						let _ = help();
					}
				}
			} else {
				let _ = help();
			}
		}
	}
	if fs::remove_dir_all(ROOT).is_ok() {
		println!("The dungeon collapsed!");
	}
}
```
# `utils.rs`
```rs
pub const ROOT: &str = "dungeon";
```
