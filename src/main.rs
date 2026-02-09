//! This is a dungeon crawler cli game written in Rust!
mod inventory;
mod utils;
use {
	crate::{
		inventory::{
			add,
			check,
			drop,
			list,
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
#[command(name = "")]
/// The command-line-interface used throughout the game
struct Cli {
	#[command(subcommand)]
	/// A root command
	command: Command,
}
#[derive(Subcommand)]
/// Root commands
enum Command {
	#[command(subcommand)]
	/// Inventory commands
	Inventory(Inventory),
	/// Leave the dungeon
	Quit,
}
#[derive(Subcommand)]
/// Inventory commands
enum Inventory {
	/// Add `<item>` to your inventory,
	/// with an optional increase amount
	/// (defaults to 1 if not specified)
	Add { item: String, increase: Option<u32> },
	/// Check if `<item>` is in your inventory,
	/// with an optional target amount
	/// (defaults to 1 if not specified)
	Check { item: String, target: Option<u32> },
	/// Remove `<item>` from your inventory,
	/// with an optional decrease amount
	/// (defaults to 1 if not specified)
	Drop { item: String, increase: Option<u32> },
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
