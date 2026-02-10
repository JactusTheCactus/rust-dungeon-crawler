- Use `Ron` for state
	`Ron` struct file per-file (à la Unix) for metadata,
		instead of raw text for item count.
	- e.g. Bow×1, Arrow×50:
		- Raw
			- `./bow`
				```txt
				1
				```
			- `./arrow`
				```txt
				50
				```
		- `Ron`
			- `./bow.ron`
				```rs
				Item(
					name: "Bow",
					description: "Slay your enemies from afar!",
					count: 1_u8,
				)
				```
			- `./arrow.ron`
				```rs
				Item(
					name: "Arrow",
					description: "Pointy!",
					count: 50_u8,
				)
				```
