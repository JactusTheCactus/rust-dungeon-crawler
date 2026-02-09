use clap::Subcommand;
#[derive(Subcommand)]
pub enum Inventory {
	/// Add items to your inventory,
	/// with an optional increase amount
	/// (defaults to 1)
	#[command(alias = "a")]
	Add {
		/// The item(s) you want to add to your inventory
		item: String,
		/// The amount of items to add (defaults to 1)
		#[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
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
		#[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
		target: u8,
	},
	/// Remove items from your inventory,
	/// with an optional decrease amount
	/// (defaults to 1)
	#[command(alias = "d")]
	Drop {
		/// The item(s) you want to drop from your inventory
		item: String,
		/// The amount of items to add (defaults to 1)
		#[arg(default_value_t = 1, value_parser = clap::value_parser!(u8).range(1..))]
		decrease: u8,
	},
	/// List all items in your inventory
	#[command(alias = "l")]
	List,
}
