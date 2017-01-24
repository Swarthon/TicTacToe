use std::io;
use std::process;

struct Node {
	child	: Vec<Node>,
	terrain	: [[char;3];3],
}

fn print_terrain(terrain : &[[char;3];3]) {
	println!("-------------");
	for line in terrain.iter() {
		for case in line.iter() {
			print!("| {} ", case);
		}
		println!("|");
		println!("-------------");
	}
	println!("");
}

fn input() -> (usize, usize) {
	(read_case("x : "), read_case("y : "))
}

fn read_case(msg : &str) -> usize {
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

fn play(terrain : &mut [[char;3];3], x : usize, y : usize, player : &mut char){
	if terrain[y][x] != ' ' {
		return;
	}
	terrain[y][x] = *player;
	if *player == 'X' {
		*player = 'O';
	}
	else {
		*player = 'X';
	}
}

fn ai_play(terrain : &mut [[char;3];3]){
	println!("L'IA joue ...");
}

fn test_win(terrain : &[[char;3];3]){
	for line in terrain {
		if line[0] == line[1] && line[0] == line[2] && line[0] != ' ' {
			win(line[0]);
		}
	}
	for y in 0..terrain.len() {
		if terrain[0][y] == terrain[1][y] && terrain[0][y] == terrain[2][y] && terrain[0][y] != ' ' {
			win(terrain[0][y]);
		}
	}
	if terrain[0][0] == terrain[1][1] && terrain[0][0] == terrain[2][2]  && terrain[0][0] != ' '{
		win(terrain[0][0]);
	}
	if terrain[0][2] == terrain[1][1] && terrain[0][2] == terrain[2][0]  && terrain[0][2] != ' '{
		win(terrain[0][2]);
	}

	let mut is_end : bool = true;
	for line in terrain {
		for case in line {
			if *case == ' ' {
				is_end = false;
			}
		}
	}
	if is_end {
		end();
	}

}

fn win(player : char){
	println!("Le joueur {} a gagné !!", player);
	process::exit(0);
}
fn end(){
	println!("La partie se termine sur un match nul");
	process::exit(0);
}

fn choose_mod(play_mod : &mut bool){
	let mut input = String::new();
	println!("Comment voulez vous jouez ?\n\tSolo (s)\tMultijoueur (m)");
	io::stdin().read_line(&mut input)
		.expect("failed to read line");
	*play_mod = match input.trim() {
	    "s" => false,
		"m"	=> true,
		_	=> return choose_mod(play_mod),
	};
}

fn ai_calculate(node : &mut Node){
}

fn main() {
	let mut terrain = [[' ';3];3];
	let mut player = 'X';

	let mut ai_data = Node{child : Vec::new(), terrain : terrain};
	ai_calculate(&mut ai_data);

	let mut play_mod = false;
	choose_mod(&mut play_mod);

	loop {
		print_terrain(&terrain);
		test_win(&terrain);
		let (x, y)  = input();
		play(&mut terrain, x, y, &mut player);
		if play_mod {
			ai_play(&mut terrain);
		}
	}
}

// TODO End it
// Maybe add a return value to ai_calculate with recursivity
// Add a previous Node parameter that can be used to search back for data
