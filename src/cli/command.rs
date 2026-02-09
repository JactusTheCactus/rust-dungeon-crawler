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
