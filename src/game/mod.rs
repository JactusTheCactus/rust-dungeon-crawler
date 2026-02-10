mod inv;
use {
	crate::{
		ROOT,
		cli::inv::{
			InvCmd,
			InvCmd::{
				Add,
				Check,
				Drop,
				List,
			},
		},
		game::inv::{
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
pub fn inventory(command: InvCmd) {
	match command {
		Add { item, increase } => add(item, increase),
		Check { item, target } => check(item, target),
		Drop { item, decrease } => drop(item, decrease),
		List => list(),
	}
}
