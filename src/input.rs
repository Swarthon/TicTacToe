use std::io;

pub fn input() -> (usize, usize) {
	(read_case("x : "), read_case("y : "))
}

pub fn read_case(msg : &str) -> usize {
	let mut play = String::new();
	print!("{}\n", msg);
	io::stdin().read_line(&mut play)
		.expect("failed to read line");
	let case : usize = match play.trim().parse(){
		Ok(num)	=> match num > 3 {
			true	=> return read_case(msg),
			false	=> num,
		},
		Err(_)	=> return read_case(msg),
	};
	case - 1
}
