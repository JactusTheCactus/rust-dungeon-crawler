mod add;
mod check;
mod drop;
mod list;
mod read_n;
use {
	add::main as add,
	check::main as check,
	drop::main as drop,
	list::main as list,
};
pub fn run(args: &mut Vec<&str>) -> Result<bool, ()> {
	if let Some(cmd) = args.get(1) {
		let num_args = args.len() - 2;
		match *cmd {
			| "a" | "add" => {
				if let Some(item) = args.get(2) {
					if let Some(i) = args.get(3) {
						add(item, Some(i.parse::<i32>().unwrap())).expect("Invalid inventory add");
					} else {
						add(item, None).expect("Invalid inventory add");
					}
				} else {
					println!("'inventory add' takes 1-2 arguments (provided {num_args}).")
				}
			}
			| "d" | "drop" => {
				if let Some(item) = args.get(2) {
					if let Some(i) = args.get(3) {
						drop(item, Some(i.parse::<i32>().unwrap()))
							.expect("Invalid inventory drop");
					} else {
						drop(item, None).expect("Invalid inventory drop");
					}
				} else {
					println!("'inventory drop' takes 1-2 arguments (provided {num_args}).")
				}
			}
			| "c" | "check" => {
				if let Some(i) = args.get(2) {
					if let Some(t) = args.get(3) {
						let target = t.parse::<i32>().unwrap();
						check(i, Some(target)).expect("Invalid inventory check");
					} else {
						check(i, None).expect("Invalid inventory check");
					}
				} else {
					println!("'inventory check' takes 1-2 arguments (provided {num_args}).")
				}
			}
			| "l" | "list" => {
				list().expect("Invalid inventory list");
			}
			| _ => println!("'{cmd}' is an invalid argument. Try again."),
		}
	} else {
		list().expect("Invalid inventory list");
	}
	Ok(true)
}
