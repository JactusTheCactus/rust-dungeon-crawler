Thoughts?
[4KB] ./
├── [4KB] src/
│   ├── [4KB] cli/
│   │   ├── [196B] cli.rs
│   │   ├── [270B] command.rs
│   │   ├── [1.2KB] inventory.rs
│   │   └── [49B] mod.rs
│   ├── [4KB] game/
│   │   ├── [2.1KB] inventory.rs
│   │   └── [692B] mod.rs
│   ├── [353B] lib.rs
│   └── [939B] main.rs
└── [56B] TODO.md
# `src/cli/cli.rs`
```rs
use {
	crate::cli::command::Command,
	clap::Parser,
};
#[derive(Parser)]
#[command(name = "")]
/// Player commands
pub struct Cli {
	#[command(subcommand)]
	/// A command
	pub command: Command,
}
```
# `src/cli/command.rs`
```rs
use {
	crate::cli::inventory::Inventory,
	clap::Subcommand,
};
#[derive(Subcommand)]
/// Commands
pub enum Command {
	#[command(subcommand)]
	#[command(alias = "i")]
	/// Inventory commands
	Inventory(Inventory),
	#[command(alias = "q")]
	/// Leave the dungeon
	Quit,
}
```
# `src/cli/inventory.rs`
```rs
use clap::Subcommand;
#[derive(Subcommand)]
pub enum Inventory {
	/// Add items to your inventory,
	/// with an optional increase amount
	/// (defaults to 1)
	#[command(alias = "a")]
	Add {
		/// The item(s) you want to add to your inventory
		item: String,
		/// The amount of items to add (defaults to 1)
		#[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
		increase: u8,
	},
	/// Check if an item is in your inventory,
	/// with an optional target amount
	/// (defaults to 1)
	#[command(alias = "c")]
	Check {
		/// The item(s) you want to check your inventory for
		item: String,
		/// The amount of items to check (defaults to 1)
		#[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
		target: u8,
	},
	/// Remove items from your inventory,
	/// with an optional decrease amount
	/// (defaults to 1)
	#[command(alias = "d")]
	Drop {
		/// The item(s) you want to drop from your inventory
		item: String,
		/// The amount of items to add (defaults to 1)
		#[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
		decrease: u8,
	},
	/// List all items in your inventory
	#[command(alias = "l")]
	List,
}
```
# `src/cli/mod.rs`
```rs
pub mod cli;
pub mod command;
pub mod inventory;
```
# `src/game/inventory.rs`
```rs
use {
	crate::{
		MAX,
		ROOT,
		cleanse,
		read_n,
	},
	std::{
		fs::{
			read_dir,
			remove_file,
			write,
		},
		path::Path,
	},
};
pub fn add(mut item: String, increase: u8) {
	item = cleanse(item);
	let path = Path::new(ROOT).join(".state/items").join(&item);
	let mut count = read_n(&path.display().to_string());
	count += increase;
	if count >= MAX {
		if count == MAX {
			println!("This slot is full");
		} else {
			println!("This slot is now full");
		}
		count = MAX;
	}
	if let Err(e) = write(&path, count.to_string()) {
		eprintln!("Failed to write to file: {e}");
	}
	println!("{item}×{count}");
}
pub fn check(mut item: String, target: u8) {
	item = cleanse(item);
	let path = Path::new(ROOT).join(".state/items").join(&item);
	let count = read_n(&path.display().to_string());
	if count >= target {
		println!("You have {item}×{target} ({count})");
	} else {
		println!("You do not have {item}×{target} ({count})");
	}
}
pub fn drop(mut item: String, decrease: u8) {
	item = cleanse(item);
	let path = Path::new(ROOT).join(".state/items").join(&item);
	let mut count = read_n(&path.display().to_string());
	if count == 0 {
		println!("You have nothing to drop");
	} else if count <= decrease {
		if let Err(e) = remove_file(&path) {
			eprintln!("Failed to remove file: {e}")
		}
		count = 0;
	} else {
		count -= decrease;
		if let Err(e) = write(&path, count.to_string()) {
			eprintln!("Failed to write to file: {e}");
		}
	}
	println!("{item}×{count}");
}
pub fn list() {
	let path = Path::new(ROOT).join(".state/items");
	if let Ok(items) = read_dir(&path) {
		let mut item_vec: Vec<(String, u8)> = Vec::new();
		for i in items {
			if let Ok(entry) = i {
				let item = entry.path();
				let dir = format!("{}/", &path.display().to_string()).to_string();
				item_vec.push((
					item.display().to_string().replace(&dir, ""),
					read_n(&item.display().to_string()),
				));
			}
		}
		item_vec.sort_by(|a, b| a.0.cmp(&b.0));
		if item_vec.is_empty() {
			println!("Your inventory is empty...")
		} else {
			for (item, count) in item_vec {
				println!("{item}×{count}");
			}
		}
	}
}
```
# `src/game/mod.rs`
```rs
mod inventory;
use {
	crate::{
		ROOT,
		cli::inventory::{
			Inventory,
			Inventory::{
				Add,
				Check,
				Drop,
				List,
			},
		},
		game::inventory::{
			add,
			check,
			drop,
			list,
		},
	},
	std::{
		fs::remove_dir_all,
		process::exit,
	},
};
pub fn quit() {
	if remove_dir_all(ROOT).is_ok() {
		println!("You escaped the dungeon before it collapsed!");
		exit(0);
	} else {
		eprintln!("The dungeon collapsed with you inside...");
		exit(1)
	}
}
pub fn inventory(command: Inventory) {
	match command {
		Add { item, increase } => add(item, increase),
		Check { item, target } => check(item, target),
		Drop { item, decrease } => drop(item, decrease),
		List => list(),
	}
}
```
# `src/lib.rs`
```rs
pub mod cli;
pub mod game;
use std::fs::read_to_string;
pub const ROOT: &str = "dungeon";
pub const MAX: u8 = 64;
pub fn read_n(path: &str) -> u8 {
	if let Some(str) = read_to_string(&path).ok() {
		if let Some(n) = str.parse::<u8>().ok() {
			return n;
		}
	}
	0
}
pub fn cleanse(input: String) -> String {
	input.replace("/", "_").replace(".", "_")
}
```
# `src/main.rs`
```rs
use {
	clap_repl::{
		ClapEditor,
		reedline::{
			DefaultPrompt,
			DefaultPromptSegment::Basic,
			FileBackedHistory,
		},
	},
	rust_dungeon_crawler::{
		ROOT,
		cli::{
			cli::Cli,
			command::Command::{
				Inventory,
				Quit,
			},
		},
		game::{
			inventory,
			quit,
		},
	},
	std::fs::create_dir_all,
};
fn main() {
	for dir in [".state/items"] {
		if let Err(e) = create_dir_all(format!("{ROOT}/{dir}")) {
			eprintln!("Failed to create directory: {e}")
		}
	}
	let prompt = DefaultPrompt {
		left_prompt: Basic("Dungeon".to_string()),
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
		Inventory(command) => inventory(command),
		Quit => quit(),
	});
}
```
# `TODO.md`
```md
- [ ] use `RON` for state instead of files (à la Unix)
```
