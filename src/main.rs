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
