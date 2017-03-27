use std::io;
use rand;
use rand::Rng;
use clap;

use ai;
use basic;
use input;

/// Return either true if it is a multiplayer game or false if it is a singleplayer game
fn choose_mod() -> bool{
	let mut input = String::new();
	println!("Comment voulez vous jouer ?\n\tSolo (s)\tMultijoueur (m)");
	io::stdin().read_line(&mut input)
		.expect("failed to read line");
	match input.trim() {
		"s" => false,
		"m" => true,
		_   => return choose_mod(),
	}
}

pub fn main(matches : clap::ArgMatches){
	let mut terrain = [[' ';3];3];
	let mut player = match rand::thread_rng().gen_range(0, 2){0 => 'O', _ => 'X'};

	// Choose the mode
	let play_mode;
	if matches.is_present("solo") {
		play_mode = true;
	}
	else if matches.is_present("multiplayer") {
		play_mode = false;
	}
	else {
		play_mode = choose_mod();
	}
	// ---

	// Initialize the AI data
	let mut ai_data : ai::Node = ai::Node {terrain : terrain, child : Vec::new(), player : player, x_win : 0, o_win : 0, play : (0,0)};
	if play_mode {
		ai_data = ai::begin(&player);
	}
	let mut ai_played_node = Vec::new();
	// ---

	// Game loop
	loop {
		basic::print_terrain(&terrain);

		// Player
		let (x, y) = input::input();
		let played = basic::play(&mut terrain, x, y, &mut player);
		basic::test_win_with_end(&terrain);
		// ---

		// AI
		if play_mode && played {
			let u = ai::update(x,y, &ai::get_node(&ai_data, &ai_played_node));
			ai_played_node.push(u);
			let u = ai::play(&mut terrain, &ai::get_node(&ai_data, &ai_played_node), &mut player);
			ai_played_node.push(u);
			basic::test_win_with_end(&terrain);
		}
		// ---
	}
	// ---
}
