mod add;
mod drop;
mod list;
use {
	add::main as add,
	drop::main as drop,
	list::main as list,
};
pub fn run(args: &mut Vec<&str>) {
	let arg_len = args.len() - 1;
	if arg_len > 0 {
		match args[1] {
			| "a" | "add" => add(args, arg_len),
			| "d" | "drop" => drop(args, arg_len),
			| _ => println!("'{}' is an invalid argument. Try again.", args[1]),
		}
	} else {
		list();
	}
}
