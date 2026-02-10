# Thoughts?
```tree
./
├── src/
│   ├── cli/
│   │   ├── cli.rs
│   │   ├── command.rs
│   │   ├── inventory.rs
│   │   └── mod.rs
│   ├── game/
│   │   ├── inventory/
│   │   │   ├── store/
│   │   │   │   ├── func.rs
│   │   │   │   └── mod.rs
│   │   │   └── mod.rs
│   │   └── mod.rs
│   ├── lib.rs
│   └── main.rs
└── TODO.md
```
## `src/cli/cli.rs`
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
## `src/cli/command.rs`
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
## `src/cli/inventory.rs`
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
		#[arg(default_value_t = 1_u8, value_parser = clap::value_parser!(u8).range(1_i64..))]
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
		#[arg(default_value_t = 1_u8, value_parser = clap::value_parser!(u8).range(1_i64..))]
		target: u8,
	},
	/// Remove items from your inventory,
	/// with an optional decrease amount
	/// (defaults to 1)
	#[command(alias = "d")]
	Drop {
		/// The item(s) you want to drop from your inventory
		item: String,
		/// The amount of items to drop (defaults to 1)
		#[arg(default_value_t = 1_u8, value_parser = clap::value_parser!(u8).range(1_i64..))]
		decrease: u8,
	},
	/// List all items in your inventory
	#[command(alias = "l")]
	List,
}
```
## `src/cli/mod.rs`
```rs
pub mod cli;
pub mod command;
pub(super) mod inventory;
```
## `src/game/inventory/mod.rs`
```rs
mod store;
use crate::{
	cleanse,
	game::inventory::store::{
		InventoryStore,
		Item,
		get_item_path,
	},
	read_n,
};
pub(super) fn add(mut item: String, increase: u8) {
	let max: u8 = 1 << 6;
	item = cleanse(item);
	let path = get_item_path(&item);
	let mut count: u8 = read_n(&path);
	let old = count;
	count = (old + increase).min(max);
	if old == max {
		println!("This slot is full");
	} else if count == max {
		println!("This slot is now full");
	}
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
	let inv = InventoryStore {};
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
```
## `src/game/inventory/store/func.rs`
```rs
use {
	crate::{
		ROOT,
		cleanse,
		game::inventory::store::{
			InventoryStore,
			Item,
			get_item_path,
		},
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
pub(super) fn get(mut item: String) -> Item {
	item = cleanse(item);
	let path = &Path::new(ROOT)
		.join(".state")
		.join("items")
		.join(&cleanse(item.to_string()));
	let count = read_n(path);
	return Item {
		id: item.to_string(),
		count: count,
		path: path.to_path_buf(),
	};
}
pub(super) fn set(item: &String, count: u8) {
	let path = get_item_path(&item);
	if let Err(e) = write(&path, count.to_string()) {
		eprintln!("Failed to write to file: {e}");
	}
}
pub(super) fn remove(item: String) {
	let Item {
		id: _,
		count: _,
		path,
	} = InventoryStore {}.get(item);
	if let Err(e) = remove_file(&path) {
		eprintln!("Failed to remove file: {e}")
	}
}
pub(super) fn list() -> Vec<(String, u8)> {
	let path = Path::new(ROOT).join(".state").join("items");
	let mut item_vec: Vec<(String, u8)> = Vec::new();
	if let Ok(items) = read_dir(&path) {
		for i in items {
			if let Ok(entry) = i {
				item_vec.push((
					entry.file_name().to_string_lossy().into_owned(),
					read_n(&entry.path()),
				));
			}
		}
		item_vec.sort_by(|(a, _), (b, _)| a.cmp(&b));
	}
	item_vec
}
```
## `src/game/inventory/store/mod.rs`
```rs
mod func;
use {
	crate::{
		ROOT,
		game::inventory::store::func::{
			get,
			list,
			remove,
			set,
		},
	},
	std::path::{
		Path,
		PathBuf,
	},
};
pub(super) fn get_item_path(item: &String) -> PathBuf {
	Path::new(ROOT).join(".state/items").join(&item)
}
pub(super) struct Item {
	pub(super) id: String,
	pub(super) count: u8,
	pub(super) path: PathBuf,
}
pub(super) struct InventoryStore {}
impl InventoryStore {
	pub(super) fn get(&self, item: String) -> Item {
		get(item)
	}
	pub(super) fn set(&self, item: &String, count: u8) {
		set(item, count)
	}
	pub(super) fn remove(&self, item: String) {
		remove(item)
	}
	pub(super) fn list(&self) -> Vec<(String, u8)> {
		list()
	}
}
```
## `src/game/mod.rs`
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
		exit(0_i32);
	} else {
		eprintln!("The dungeon collapsed with you inside...");
		exit(1_i32)
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
## `src/lib.rs`
```rs
pub mod cli;
pub mod game;
use {
	once_cell::sync::Lazy,
	regex::Regex,
	std::{
		fs::read_to_string,
		path::Path,
	},
};
pub const ROOT: &str = "dungeon";
pub(crate) fn read_n(path: &Path) -> u8 {
	read_to_string(path)
		.ok()
		.and_then(|s| s.parse().ok())
		.unwrap_or(0)
}
static CLEANSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[/.\s]").unwrap());
pub(crate) fn cleanse(input: String) -> String {
	CLEANSE_RE
		.replace_all(input.as_str(), "_")
		.to_string()
		.to_lowercase()
}
```
## `src/main.rs`
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
	ClapEditor::<Cli>::builder()
		.with_prompt(Box::new(DefaultPrompt {
			left_prompt: Basic("Dungeon".to_string()),
			..DefaultPrompt::default()
		}))
		.with_editor_hook(|reed| {
			reed.with_history(Box::new(
				FileBackedHistory::with_file(
					10000_usize,
					"/tmp/rust-dungeon-crawler-history".into(),
				)
				.unwrap(),
			))
		})
		.build()
		.repl(|cmd| match cmd.command {
			Inventory(command) => inventory(command),
			Quit => quit(),
		});
}
```
## `TODO.md`
```md
- Instead of files (à la Unix), use `RON` for state
```
