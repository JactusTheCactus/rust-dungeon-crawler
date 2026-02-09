mod inventory;
use {
	crate::{
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
		utils::ROOT,
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
