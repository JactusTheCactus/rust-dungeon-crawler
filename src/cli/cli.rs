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
