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
			cli::Cli,
			command::Command::{
				Inventory,
				Quit,
			},
		},
		game::{
			inventory,
			quit,
		},
	},
	std::fs::create_dir_all,
};
fn main() {
	for dir in [".state/items"] {
		if let Err(e) = create_dir_all(format!("{ROOT}/{dir}")) {
			eprintln!("Failed to create directory: {e}")
		}
	}
	ClapEditor::<Cli>::builder()
		.with_prompt(Box::new(DefaultPrompt {
			left_prompt: Basic("Dungeon".to_string()),
			..DefaultPrompt::default()
		}))
		.with_editor_hook(|reed| {
			reed.with_history(Box::new(
				FileBackedHistory::with_file(
					10000_usize,
					"/tmp/rust-dungeon-crawler-history".into(),
				)
				.unwrap(),
			))
		})
		.build()
		.repl(|cmd| match cmd.command {
			Inventory(command) => inventory(command),
			Quit => quit(),
		});
}
