Thoughts?
# `help.rs`
```rs
struct Command {
	cmd: String,
	text: &'static str,
}
pub fn main() {
	let i = "i[nventory]";
	let help: &[Command] = &[
		Command {
			cmd: i.to_string(),
			text: "list all items in to your inventory",
		},
		Command {
			cmd: i.to_string(),
			text: "list all items in to your inventory",
		},
		Command {
			cmd: format!("{i} a[dd] <item>"),
			text: "add <item> to your inventory",
		},
		Command {
			cmd: format!("{i} d[rop] <item>"),
			text: "remove <item> from your inventory",
		},
	];
	println!("Commands:");
	for Command { cmd, text } in help {
		println!("\t{cmd}\n\t\t{text}");
	}
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
pub fn main(args: &mut Vec<&str>) {
	if args.len() == 3 {
		if let Some(item) = args.get(2) {
			let path = format!("{ROOT}/.state/items/{item}");
			let mut count: i32 = read_n(&path);
			if count <= 1 {
				let _ = fs::remove_file(&path);
				count = 0;
			} else {
				count -= 1;
				let _ = fs::write(&path, count.to_string());
			}
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
```
# `inventory/list.rs`
```rs
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
```
# `inventory/mod.rs`
```rs
mod add;
mod drop;
mod list;
mod read_n;
use {
	add::main as add,
	drop::main as drop,
	list::main as list,
};
pub fn run(args: &mut Vec<&str>) {
	if args.len() > 1 {
		match args[1] {
			| "a" | "add" => add(args),
			| "d" | "drop" => drop(args),
			| _ => println!("'{}' is an invalid argument. Try again.", args[1]),
		}
	} else {
		list();
	}
}
```
# `inventory/read_n.rs`
```rs
use std::fs;
pub fn main(path: &str) -> i32 {
	return fs::read_to_string(&path)
		.ok()
		.and_then(|s| s.parse::<i32>().ok())
		.unwrap_or(0);
}
```
# `main.rs`
```rs
mod help;
mod inventory;
mod utils;
use {
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
			clear();
			let mut cmd = input.trim().split_whitespace().collect::<Vec<&str>>();
			match cmd[0] {
				| "i" | "inventory" => inventory::run(&mut cmd),
				| "q" | "quit" => {
					break;
				}
				| _ => help(),
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
pub fn clear() {
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
```
