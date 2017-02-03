extern crate rand;
extern crate clap;

pub mod input;
use input::*;
pub mod basic;
use basic::*;
pub mod ai;
use ai::*;

use std::io;
use rand::Rng;
use clap::{App,Arg};

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

fn main() {
	let matches = App::new("Jeu de Morpion")
						.version("0.1.1")
	            		.author("Swarthon <swarthon.gokan@gmail.com>")
	                	.about("Jeu de Morpion | Tic Tac Toe Game")
						.arg(Arg::with_name("solo")
	                    	.short("s")
	                    	.long("solo")
							.help("Play a solo game")
							.conflicts_with("multiplayer"))
	 	                .arg(Arg::with_name("multiplayer")
	 	        			.short("m")
	 	            		.long("mulitplayer")
							.help("Play a multiplayer game")
							.conflicts_with("solo"))
	                	.get_matches();

	let mut terrain = [[' ';3];3];
	let mut player = match rand::thread_rng().gen_range(0, 2){0 => 'O', _ => 'X'};

	let play_mod;
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
			let n = ai_update(x,y, &ai_actual_node[ai_actual_node.len()-1]);
			let n = ai_play(&mut terrain, &n, &mut player);
			ai_actual_node.push(n);
		}
		test_win_with_end(&terrain);
	}
}
