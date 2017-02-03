use std::process;

pub fn play(terrain : &mut [[char;3];3], x : usize, y : usize, player : &mut char) -> bool{
	if terrain[y][x] != ' ' {
		return false;
	}
	terrain[y][x] = *player;
	*player = swap_player(player);
	true
}

pub fn win(player : char){
	println!("Le joueur {} a gagné !!", player);
	process::exit(0);
}
pub fn end(){
	println!("La partie se termine sur un match nul");
	process::exit(0);
}

pub fn swap_player(player : &char) -> char{
	match *player {
		'X'	=> 'O',
		_	=> 'X'
	}
}
pub fn print_terrain(terrain : &[[char;3];3]) {
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
pub fn test_win(terrain : &[[char;3];3]) -> char {
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
pub fn test_win_with_end(terrain : &[[char;3];3]){
	if test_win(terrain) == '0'{
		print_terrain(terrain);
		end();
	}
	if test_win(terrain) == 'X' || test_win(terrain) == 'O'{
		print_terrain(terrain);
		win(test_win(terrain));
	}
}