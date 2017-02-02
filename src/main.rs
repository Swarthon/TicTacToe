extern crate rand;
extern crate clap;

use std::io;
use std::process;
use rand::Rng;
use clap::{App,Arg};

struct Node {
	child	: Vec<Node>,
	terrain	: [[char;3];3],
	player	: char,
	x_win	: i64,
	o_win	: i64,
	play	: (usize,usize)
}

fn swap_player(player : &char) -> char{
	match *player {
		'X'	=> 'O',
		_	=> 'X'
	}
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

fn play(terrain : &mut [[char;3];3], x : usize, y : usize, player : &mut char) -> bool{
	if terrain[y][x] != ' ' {
		return false;
	}
	terrain[y][x] = *player;
	*player = swap_player(player);
	true
}

fn ai_play<'a>(terrain : &mut [[char;3];3], n : &'a Node, player : &mut char) -> &'a Node {
	let mut case = 0;
	for i in 0..n.child.len() {
		if *player == 'X' && n.child[i].o_win < n.child[case].o_win {
			case = i;
		}
		if *player == 'X' && n.child[i].o_win == n.child[case].o_win {
			if n.child[i].x_win > n.child[case].x_win {
				case = i;
			}
		}
		if *player == 'O' && n.child[i].x_win < n.child[case].x_win {
			case = i;
		}
		if *player == 'O' && n.child[i].x_win == n.child[case].x_win {
			if n.child[i].o_win > n.child[case].o_win {
				case = i;
			}
		}
	}
	play(terrain, n.child[case].play.0, n.child[case].play.1, player);
	return &n.child[case];
}

fn test_win(terrain : &[[char;3];3]) -> char {
	for line in terrain {
		if line[0] == line[1] && line[0] == line[2] && line[0] != ' ' {
			return line[0];
		}
	}
	for y in 0..terrain.len() {
		if terrain[0][y] == terrain[1][y] && terrain[0][y] == terrain[2][y] && terrain[0][y] != ' ' {
			return terrain[0][y];
		}
	}
	if terrain[0][0] == terrain[1][1] && terrain[0][0] == terrain[2][2]  && terrain[0][0] != ' '{
		return terrain[0][0];
	}
	if terrain[0][2] == terrain[1][1] && terrain[0][2] == terrain[2][0]  && terrain[0][2] != ' '{
		return terrain[0][2];
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
		return '0';
	}
	' '
}
fn test_win_with_end(terrain : &[[char;3];3]){
	if test_win(terrain) == '0'{
		print_terrain(terrain);
		end();
	}
	if test_win(terrain) == 'X' || test_win(terrain) == 'O'{
		print_terrain(terrain);
		win(test_win(terrain));
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

fn choose_mod() -> bool{
	let mut input = String::new();
	println!("Comment voulez vous jouer ?\n\tSolo (s)\tMultijoueur (m)");
	io::stdin().read_line(&mut input)
		.expect("failed to read line");
	match input.trim() {
	    "s" => false,
		"m"	=> true,
		_	=> return choose_mod(),
	}
}

fn ai_begin(player : &char) -> Node{
	println!("Calcul des possibilités en cours ...");
	let terrain = [[' ';3];3];
	let v : Vec<Node> = Vec::new();
	let mut n = Node {terrain : terrain, child : v, player : *player, x_win : 0, o_win : 0, play : (0,0)};
	for p in 0..9 {
		let child = ai_calculate_node(&n.terrain, &n.player, p);
		n.x_win += child.x_win;
		n.o_win += child.o_win;
		n.child.push(child);
	}
	n
}
fn ai_add_point(n : &mut Node, u : u8) -> bool{
	let x : usize = (u % 3) as usize;
	let y : usize = (u / 3) as usize;

	if n.terrain[y][x] != ' ' {
		return true;
	}
	else {
		n.terrain[y][x] = n.player;
		n.play = (x,y);
		let res = test_win(&n.terrain);
		if res == 'O' {
			n.o_win += 1;
			return true;
		}
		else if res == 'X' {
			n.x_win += 1;
			return true;
		}
		else if res == '0' {
			return true;
		}
	}
	false
}
fn ai_calculate_node(terrain : &[[char;3];3], player : &char, u : u8) -> Node{
	let mut n = Node {terrain : *terrain, child : Vec::new(), player : *player, x_win : 0, o_win : 0, play : (0,0)};
	let b = ai_add_point(&mut n, u);
	if b {
		return n;
	}
	for p in 0..9 {
		let child = ai_calculate_node(&n.terrain, &swap_player(player), p);
		if child.terrain != n.terrain {
			n.x_win += child.x_win;
			n.o_win += child.o_win;
			n.child.push(child);
		}
	}
	n
}
fn update_ai<'a>(x : usize, y : usize, n : &'a Node) -> &'a Node{
	let mut played_case = 0;
	for i in 0..n.child.len(){
		if n.child[i].play == (x,y){
			played_case = i;
		}
	}
	&n.child[played_case]
}

fn main() {
	let matches = App::new("Jeu de Morpion")
						.version("0.1.1")
	            		.author("Swarthon <swarthon.gokan@gmail.com>")
	                	.about("Jeu de Morpion | Tic Tac Toe Game")
						.arg(Arg::with_name("solo")
	                    	.short("s")
	                    	.long("solo")
							.help("Play a solo game"))
	 	                .arg(Arg::with_name("multiplayer")
	 	        			.short("m")
	 	            		.long("mulitplayer")
							.help("Play a multiplayer game"))
	                	.get_matches();

	let mut terrain = [[' ';3];3];
	let mut player = match rand::thread_rng().gen_range(0, 2){0 => 'O', _ => 'X'};

	let mut play_mod = false;
	if matches.is_present("solo") {
		play_mod = false;
	}
	else if matches.is_present("multiplayer") {
		play_mod = true;
	}
	else {
		play_mod = choose_mod();
	}

	let mut ai_data : Node = Node {terrain : terrain, child : Vec::new(), player : player, x_win : 0, o_win : 0, play : (0,0)};
	if play_mod {
		ai_data = ai_begin(&player);
	}
	let mut ai_actual_node = vec!(&ai_data);

	loop {
		print_terrain(&terrain);
		let (x, y)  = input();
		let played = play(&mut terrain, x, y, &mut player);
		test_win_with_end(&terrain);
		if play_mod && played {
			let n = update_ai(x,y, &ai_actual_node[ai_actual_node.len()-1]);
			let n = ai_play(&mut terrain, &n, &mut player);
			ai_actual_node.push(n);
		}
		test_win_with_end(&terrain);
	}
}
