# Thoughts?
```tree
src/
├── game/
│   ├── inv/
│   │   ├── mod.rs
│   │   └── store.rs
│   └── mod.rs
├── cli.rs
├── lib.rs
└── main.rs
```
## `cli.rs`
```rs
use clap::{
	Parser,
	Subcommand,
	value_parser,
};
#[derive(Subcommand)]
/// Commands
pub enum Command {
	#[command(subcommand)]
	#[command(alias = "i")]
	/// Inventory commands
	Inventory(InvCmd),
	#[command(alias = "q")]
	/// Leave the dungeon
	Quit,
}
#[derive(Parser)]
#[command(name = "")]
/// Player commands
pub struct Cli {
	#[command(subcommand)]
	/// A command
	pub command: Command,
}
#[derive(Subcommand)]
pub enum InvCmd {
	/// Add items to your inventory,
	/// with an optional increase amount
	/// (defaults to 1)
	#[command(alias = "a")]
	Add {
		/// The item(s) you want to add to your inventory
		item: String,
		/// The amount of items to add (defaults to 1)
		#[arg(default_value_t = 1_u8, value_parser = value_parser!(u8).range(1_i64..))]
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
		#[arg(default_value_t = 1_u8, value_parser = value_parser!(u8).range(1_i64..))]
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
		#[arg(default_value_t = 1_u8, value_parser = value_parser!(u8).range(1_i64..))]
		decrease: u8,
	},
	/// List all items in your inventory
	#[command(alias = "l")]
	List,
}
```
## `game/inv/mod.rs`
```rs
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
```
## `game/inv/store.rs`
```rs
use std::{
	fs::{
		read_dir,
		read_to_string,
		remove_file,
		write,
	},
	path::{
		Path,
		PathBuf,
	},
};
pub(super) struct Item {
	pub(super) id: String,
	pub(super) count: u8,
	pub(super) path: PathBuf,
}
pub(super) struct InventoryStore {
	pub(super) root: PathBuf,
}
impl InventoryStore {
	pub fn new(root: impl Into<PathBuf>) -> Self {
		Self { root: root.into() }
	}
	pub(super) fn item_path(&self, item: &str) -> PathBuf {
		Path::new(&self.root)
			.join(".state")
			.join("items")
			.join(&item)
	}
	pub(crate) fn read_n(&self, path: &Path) -> u8 {
		read_to_string(path)
			.ok()
			.and_then(|s| s.parse().ok())
			.unwrap_or(0)
	}
	pub(super) fn get(&self, item: &str) -> Item {
		let path = self.item_path(&item);
		let count = self.read_n(&path);
		Item {
			id: item.to_string(),
			count,
			path,
		}
	}
	pub(super) fn set(&self, item: &str, count: u8) {
		if let Err(e) = write(&self.item_path(&item), count.to_string()) {
			eprintln!("Failed to write to file: {e}");
		}
	}
	pub(super) fn remove(&self, item: &str) {
		let Item {
			id: _,
			count: _,
			path,
		} = self.get(item);
		if let Err(e) = remove_file(&path) {
			eprintln!("Failed to remove file: {e}")
		}
	}
	pub(super) fn list(&self) -> Vec<(String, u8)> {
		let path = Path::new(&self.root).join(".state").join("items");
		let mut item_vec: Vec<(String, u8)> = Vec::new();
		if let Ok(items) = read_dir(&path) {
			for i in items {
				if let Ok(entry) = i {
					item_vec.push((
						entry.file_name().to_string_lossy().into_owned(),
						self.read_n(&entry.path()),
					));
				}
			}
			item_vec.sort_by(|(a, _), (b, _)| a.cmp(&b));
		}
		item_vec
	}
}
```
## `game/mod.rs`
```rs
use {
	crate::{
		ROOT,
		cleanse,
		cli::InvCmd,
	},
	std::{
		fs::remove_dir_all,
		process::exit,
	},
};
mod inv;
pub fn quit() {
	if remove_dir_all(ROOT).is_ok() {
		println!("You escaped the dungeon before it collapsed!");
		exit(0_i32);
	} else {
		eprintln!("The dungeon collapsed with you inside...");
		exit(1_i32)
	}
}
pub fn inventory(command: InvCmd) {
	match command {
		InvCmd::Add { item, increase } => inv::add(cleanse(&item), increase),
		InvCmd::Check { item, target } => inv::check(&cleanse(&item), target),
		InvCmd::Drop { item, decrease } => inv::drop(&cleanse(&item), decrease),
		InvCmd::List => inv::list(),
	}
}
```
## `lib.rs`
```rs
use {
	once_cell::sync::Lazy,
	regex::Regex,
};
pub mod cli;
pub mod game;
pub const ROOT: &str = "dungeon";
static CLEANSE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\w-]").unwrap());
pub(crate) fn cleanse(input: &str) -> String {
	CLEANSE_RE.replace_all(input, "_").to_lowercase()
}
```
## `main.rs`
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
			Cli,
			Command,
		},
		game,
	},
	std::{
		fs::create_dir_all,
		path::Path,
	},
};
fn main() {
	for dir in [Path::new(".state").join("items")] {
		if let Err(e) = create_dir_all(Path::new(ROOT).join(dir)) {
			eprintln!("Failed to create directory: {e}")
		}
	}
	ClapEditor::<Cli>::builder()
		.with_prompt(Box::new(DefaultPrompt::new(
			Basic("Dungeon".to_string()),
			DefaultPrompt::default().right_prompt,
		)))
		.with_editor_hook(|reed| {
			reed.with_history(Box::new(
				FileBackedHistory::with_file(
					1_usize << 14_u8,
					"/tmp/rust-dungeon-crawler-history".into(),
				)
				.unwrap(),
			))
		})
		.build()
		.repl(|cmd| match cmd.command {
			Command::Inventory(command) => game::inventory(command),
			Command::Quit => game::quit(),
		});
}
```
