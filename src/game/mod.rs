use {
	crate::{
		ROOT,
		cli::inv::InvCmd,
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
		InvCmd::Add { item, increase } => inv::add(item, increase),
		InvCmd::Check { item, target } => inv::check(&item, target),
		InvCmd::Drop { item, decrease } => inv::drop(&item, decrease),
		InvCmd::List => inv::list(),
	}
}
