Thoughts?
# `inventory/add.rs`
```rs
use {
	crate::utils::{
		ROOT,
		read_n,
	},
	std::fs,
};
pub fn add(item: &str, increase: Option<i32>) -> bool {
	let max = 0b1000000;
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path);
	let mut increment = 1;
	if let Some(i) = increase {
		if i > 0 {
			increment = i
		} else {
			println!("Err: Increase must be a positive integer");
			return false;
		}
	}
	count += increment;
	if count >= max {
		count = max;
		println!("This slot is full")
	}
	let _ = fs::write(&path, count.to_string());
	println!("{item}×{count}");
	true
}
```
# `inventory/check.rs`
```rs
use crate::{
	ROOT,
	utils::read_n,
};
pub fn check(item: &str, target: Option<i32>) -> bool {
	let path = format!("{ROOT}/.state/items/{item}");
	let count = read_n(&path);
	let tar: i32;
	if let Some(t) = target {
		if t > 0 {
			tar = t
		} else {
			println!("Err: Target must be a positive integer");
			return false;
		}
	} else {
		tar = 1
	}
	if count >= tar {
		println!("You have {item}×{tar} ({count})");
		true
	} else {
		println!("You do not have {item}×{tar} ({count})");
		false
	}
}
```
# `inventory/drop.rs`
```rs
use {
	crate::utils::{
		ROOT,
		read_n,
	},
	std::fs,
};
pub fn drop(item: &str, decrease: Option<i32>) -> bool {
	let path = format!("{ROOT}/.state/items/{item}");
	let mut count = read_n(&path);
	let mut decrement = 1;
	if let Some(d) = decrease {
		if d > 0 {
			decrement = d
		} else {
			println!("Err: Decrease must be a positive integer");
			return false;
		}
	}
	if count <= decrement {
		let _ = fs::remove_file(&path);
		count = 0;
	} else {
		count -= decrement;
		let _ = fs::write(&path, count.to_string());
	}
	println!("{item}×{count}");
	true
}
```
# `inventory/list.rs`
```rs
use {
	crate::utils::ROOT,
	std::{
		collections::{
			BTreeSet,
			HashMap,
		},
		fs,
	},
};
pub fn list() -> bool {
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
	true
}
```
# `inventory/mod.rs`
```rs
pub mod add;
pub mod check;
pub mod drop;
pub mod list;
```
# `main.rs`
```rs
mod inventory;
mod utils;
use {
	crate::{
		inventory::{
			add::add,
			check::check,
			drop::drop,
			list::list,
		},
		utils::ROOT,
	},
	clap::{
		Parser,
		Subcommand,
	},
	clap_repl::{
		ClapEditor,
		reedline::{
			DefaultPrompt,
			DefaultPromptSegment,
			FileBackedHistory,
		},
	},
	std::{
		fs,
		process::exit,
	},
};
#[derive(Parser)]
struct Cli {
	#[command(subcommand)]
	command: Command,
}
#[derive(Subcommand)]
#[command(name = "Dungeon Crawler")]
enum Command {
	#[command(subcommand)]
	/// Inventory commands
	Inventory(Inventory),
	/// Leave the dungeon
	Quit,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Subcommand)]
enum Inventory {
	/// Add <item> to your inventory,
	/// with an optional increase amount
	/// (defaults to 1 if not specified)
	Add { item: String, increase: Option<i32> },
	/// Check if <item> is in your inventory,
	/// with an optional target amount
	/// (defaults to 1 if not specified)
	Check { item: String, target: Option<i32> },
	/// Remove <item> from your inventory,
	/// with an optional decrease amount
	/// (defaults to 1 if not specified)
	Drop { item: String, increase: Option<i32> },
	/// List all items in your inventory
	List,
}
fn main() {
	for dir in [".state/items"] {
		let _ = fs::create_dir_all(format!("{ROOT}/{dir}"));
	}
	let prompt = DefaultPrompt {
		left_prompt: DefaultPromptSegment::Basic("Dungeon".to_owned()),
		..DefaultPrompt::default()
	};
	let rl = ClapEditor::<Cli>::builder()
		.with_prompt(Box::new(prompt))
		.with_editor_hook(|reed| {
			reed.with_history(Box::new(
				FileBackedHistory::with_file(10000, "/tmp/rust-dungeon-crawler-history".into())
					.unwrap(),
			))
		})
		.build();
	rl.repl(|cmd| match cmd.command {
		Command::Inventory(command) => match command {
			Inventory::Add { item, increase } => {
				add(&item, increase);
			}
			Inventory::Check { item, target } => {
				check(&item, target);
			}
			Inventory::Drop { item, increase } => {
				drop(&item, increase);
			}
			Inventory::List => {
				list();
			}
		},
		Command::Quit => {
			if fs::remove_dir_all(ROOT).is_ok() {
				println!("The dungeon collapsed!");
				exit(0);
			}
		}
	});
}
```
# `utils.rs`
```rs
use std::fs;
pub const ROOT: &str = "dungeon";
pub fn read_n(path: &str) -> i32 {
	if let Some(str) = fs::read_to_string(&path).ok() {
		if let Some(n) = str.parse::<i32>().ok() {
			return n;
		}
	}
	0
}
```
