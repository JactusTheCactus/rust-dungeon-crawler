struct Command {
	cmd: String,
	text: &'static str,
}
pub fn main() {
	let i = "i[nventory]";
	let help: &[Command] = &[
		Command {
			cmd: i.to_string(),
			text: "list all items in to your inventory",
		},
		Command {
			cmd: i.to_string(),
			text: "list all items in to your inventory",
		},
		Command {
			cmd: format!("{i} a[dd] <item>"),
			text: "add <item> to your inventory",
		},
		Command {
			cmd: format!("{i} d[rop] <item>"),
			text: "remove <item> from your inventory",
		},
	];
	println!("Commands:");
	for Command { cmd, text } in help {
		println!("\t{cmd}\n\t\t{text}");
	}
}
