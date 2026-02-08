struct Command {
	cmd: String,
	text: String,
}
fn opt(r#type: &str) -> String {
	format!("with an optional {type} amount (defaults to 1 if not specified)")
}
pub fn main() -> Result<bool, ()> {
	let i = "i[nventory]";
	let help = &[
		Command {
			cmd: format!("{i} [l[ist]]"),
			text: format!("List all items in to your inventory"),
		},
		Command {
			cmd: format!("{i} a[dd] <item> [increase]"),
			text: format!("Add <item> to your inventory, {}", opt("increase")),
		},
		Command {
			cmd: format!("{i} c[heck] <item> [target]"),
			text: format!("Check if <item> is in your inventory, {}", opt("target")),
		},
		Command {
			cmd: format!("{i} d[rop] <item> [decrease]"),
			text: format!("Remove <item> from your inventory, {}", opt("decrease")),
		},
	];
	println!("Commands:");
	for Command { cmd, text } in help {
		println!("\t{cmd}\n\t\t{text}");
	}
	Ok(true)
}
