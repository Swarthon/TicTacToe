/// Enable the basic stuff coming from basic.rs which provide some basic functions as play and print_terrain
use basic;

/// Struct representing a Node
///
/// The more basic part of the tree of plays composing the Tic Tac Toe Game
#[derive(PartialEq, Eq)]
pub struct Node {
	/// All the next Nodes
	pub child	: Vec<Node>,
	/// The actual terrain
	pub terrain	: [[char;3];3],
	/// The player who played at last
	pub player	: char,
	/// The number of wins for X
	pub x_win	: i64,
	/// The number of wins for O
	pub o_win	: i64,
	/// The last play which creates this Node
	pub play	: (usize,usize)
}

/// Return the played Node
///
/// Take the `terrain` and the actual Node `n` and modify it by playing with `player`
pub fn play<'a>(terrain : &mut [[char;3];3], n : &'a Node, player : &mut char) -> usize {
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
	basic::play(terrain, n.child[case].play.0, n.child[case].play.1, player);
	case
}

/// Return the first Node of the playing tree
///
/// Take the first `player` to play and generate a tree
pub fn begin(player : &char) -> Node {
	println!("Calcul des possibilités en cours ..");
	let terrain = [[' ';3];3];
	let v : Vec<Node> = Vec::new();
	let mut n = Node {terrain : terrain, child : v, player : *player, x_win : 0, o_win : 0, play : (0,0)};
	for p in 0..9 {
		let child = calculate_node(&n.terrain, &n.player, p);
		n.x_win += child.x_win;
		n.o_win += child.o_win;
		n.child.push(child);
	}
	n
}

/// Return either true if the Node has no child or false if we should continue
///
/// Take the actual Node `n` and the actual case `u`
///
/// `u` is exprimed in the format `y * 3 + x` and is included between 0 and 9
pub fn add_point(n : &mut Node, u : u8) -> bool{
	let x : usize = (u % 3) as usize;
	let y : usize = (u / 3) as usize;

	if n.terrain[y][x] != ' ' {
		return true;
	}
	else {
		n.terrain[y][x] = n.player;
		n.play = (x,y);
		let res = basic::test_win(&n.terrain);
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

/// Return the calculated Node
///
/// Take the actual `terrain`, the actual `player` and the actual case `u`
///
/// `u` is exprimed in the format `y * 3 + x` and is included between 0 and 9
pub fn calculate_node(terrain : &[[char;3];3], player : &char, u : u8) -> Node {
	let mut n = Node {terrain : *terrain, child : Vec::new(), player : *player, x_win : 0, o_win : 0, play : (0,0)};
	let b = add_point(&mut n, u);
	if b {
		return n;
	}
	for p in 0..9 {
		let child = calculate_node(&n.terrain, &basic::swap_player(player), p);
		if child.terrain != n.terrain {
			n.x_win += child.x_win;
			n.o_win += child.o_win;
			n.child.push(child);
		}
	}
	n
}

/// Return the updated Node
///
/// Take the played cases `x`, `y` and the actual Node `n`
///
/// It updates the tree, by using the play of the player and by parsing `n`
pub fn update<'a>(x : usize, y : usize, n : &'a Node) -> usize {
	let mut played_case = 0;
	for i in 0..n.child.len(){
		if n.child[i].play == (x,y){
			played_case = i;
		}
	}
	played_case
}

/// Return the node following the `path`
///
/// Take the root node `parent` and the path to the node `path`
pub fn get_node<'a>(parent : &'a Node, path : &Vec<usize>) -> &'a Node {
	if path.len() == 0 {
		return parent;
	}
	return get_node(&parent.child[path[0]], &path[1..].to_vec());
}
