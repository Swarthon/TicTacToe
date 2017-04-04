extern crate rand;
extern crate clap;

// piston
#[macro_use]
extern crate conrod;
extern crate find_folder;

mod input;
mod basic;
mod ai;
mod graphics;
mod text;

use clap::{App,Arg};

fn main() {
	let matches = App::new("Tic Tac Toe Game")
		.version("0.1.1")
		.author("Swarthon <swarthon.gokan@gmail.com>")
		.about("Tic Tac Toe Game")
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
		.arg(Arg::with_name("text")
	 	        .short("t")
			.long("text")
			.help("Play the game in text")
			.conflicts_with("graphics"))
		.arg(Arg::with_name("graphics")
	 	        .short("g")
			.long("graphics")
			.help("Play the game with 2D graphics")
			.conflicts_with("text"))
	        .get_matches();

	if matches.is_present("text") {
		text::main(matches);
	}
	else {
		graphics::main();
	}


}
