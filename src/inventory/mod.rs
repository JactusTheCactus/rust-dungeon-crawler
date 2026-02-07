mod add;
mod drop;
mod list;
mod read_n;
use {
	add::main as add,
	drop::main as drop,
	list::main as list,
};
pub fn run(args: &mut Vec<&str>) {
	if args.len() > 1 {
		match args[1] {
			| "a" | "add" => add(args),
			| "d" | "drop" => drop(args),
			| _ => println!("'{}' is an invalid argument. Try again.", args[1]),
		}
	} else {
		list();
	}
}
