use {
	clap_repl::{
		ClapEditor,
		reedline::{
			DefaultPrompt,
			DefaultPromptSegment::Basic,
			FileBackedHistory,
		},
	},
	rust_dungeon_crawler::{
		ROOT,
		cli::{
			Cli,
			Command,
		},
		game,
	},
	std::{
		fs::create_dir_all,
		path::Path,
	},
};
fn main() {
	for dir in [Path::new(".state").join("items")] {
		if let Err(e) = create_dir_all(Path::new(ROOT).join(dir)) {
			eprintln!("Failed to create directory: {e}")
		}
	}
	ClapEditor::<Cli>::builder()
		.with_prompt(Box::new(DefaultPrompt::new(
			Basic("Dungeon".to_string()),
			DefaultPrompt::default().right_prompt,
		)))
		.with_editor_hook(|reed| {
			reed.with_history(Box::new(
				FileBackedHistory::with_file(
					1_usize << 14_u8,
					"/tmp/rust-dungeon-crawler-history".into(),
				)
				.unwrap(),
			))
		})
		.build()
		.repl(|cmd| match cmd.command {
			Command::Inventory(command) => game::inventory(command),
			Command::Quit => game::quit(),
		});
}
