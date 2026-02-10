use {
	crate::cli::inv::InvCmd,
	clap::Subcommand,
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
