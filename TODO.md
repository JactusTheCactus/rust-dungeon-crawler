# Use Ron for state
1 Ron struct file per-file (à la Unix) for metadata, instead of raw text for item count. e.g.:
## Raw
### ./bow
```txt
1
```
### ./arrow
```txt
50
```
## Ron
### ./bow.ron
```rs
Item(
	name: "Bow",
	description: "Slay your enemies from afar!",
	effects: ["triple"],
	count: 1_u8,
	max_shift: 4_u8,
)
```
### ./arrow.ron
```rs
Item(
	name: "Arrow",
	description: "Pointy!",
	effects: ["flame"],
	count: 50_u8,
	max_shift: 6_u8,
)
```
