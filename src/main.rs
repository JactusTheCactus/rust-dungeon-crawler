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
