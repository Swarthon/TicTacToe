#![allow(dead_code)]

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
	/// The player who win
	pub win 	: char,
	/// The last play which creates this Node
	pub play	: (usize,usize)
}

impl Node {
	pub fn new() -> Node {
		Node { child : Vec::new(), terrain : [[' ';3];3], player : 'X', win : ' ', play : (0,0) }
	}
}

pub struct NodeBuilder {
	child	: Vec<Node>,
	terrain	: [[char;3];3],
	player	: char,
	win 	: char,
	play	: (usize,usize)
}

impl NodeBuilder {
	pub fn new () -> NodeBuilder {
		NodeBuilder { child : Vec::new(), terrain : [[' ';3];3], player : 'X', win : ' ', play : (0,0) }
	}
	/*pub fn child (&mut self, vec : Vec<Node>) -> &mut NodeBuilder {
		self.child = vec;
		self
	}*/
	pub fn terrain (&mut self, terrain : [[char;3];3]) -> &mut NodeBuilder {
		self.terrain = terrain;
		self
	}
	pub fn player (&mut self, player : char) -> &mut NodeBuilder {
		self.player = player;
		self
	}
	pub fn win (&mut self, win : char) -> &mut NodeBuilder {
		self.win = win;
		self
	}
	pub fn play (&mut self, play : (usize,usize)) -> &mut NodeBuilder {
		self.play = play;
		self
	}
	pub fn finalize(&self) -> Node {
		Node { child : Vec::new(), terrain : self.terrain, player : self.player, win : self.win, play : self.play }
	}
}

/// Return either a `node` is safe for a specific `player`
///
/// Take a `node` and a `player`
pub fn is_safe(node : &Node, player : &char) -> bool {
	if node.player == *player {
		if node.win == '0' || node.win == *player {
			return true;
		}
		for i in 0..node.child.len() {
			if !is_safe(&node.child[i],player) {
				return false;
			}
		}
		true
	}
	else {
		if node.win == basic::swap_player(player) {
			return false;
		}
		if node.win == '0' {
			return true;
		}
		for i in 0..node.child.len() {
			if is_safe(&node.child[i],player) {
				return true;
			}
		}
		false
	}
}

/// Return the played Node
///
/// Take the `terrain` and the actual Node `n` and modify it by playing with `player`
pub fn play<'a>(terrain : &mut [[char;3];3], n : &'a Node, player : &mut char) -> usize {
	let mut index = 0;
	for i in 0..n.child.len() {
		if is_safe(&n.child[i], player) {
			index = i;
			break;
		}
	}
	basic::play(terrain, n.child[index].play.0, n.child[index].play.1, player);
	index
}

/// Return the first Node of the playing tree
///
/// Take the first `player` to play and generate a tree
pub fn begin(player : &char) -> Node {
	let terrain = [[' ';3];3];
	let v : Vec<Node> = Vec::new();
	let mut n = Node {terrain : terrain, child : v, player : *player, win : ' ', play : (0,0)};
	for p in 0..9 {
		let child = calculate_node(&n.terrain, &n.player, p);
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
		if res == 'O' || res == 'X' || res == '0' {
			n.win = res;
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
	let mut n = Node {terrain : *terrain, child : Vec::new(), player : *player, win : ' ', play : (0,0)};
	let b = add_point(&mut n, u);
	if b {
		return n;
	}
	for p in 0..9 {
		let child = calculate_node(&n.terrain, &basic::swap_player(player), p);
		if child.terrain != n.terrain {
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

/// Return the `node` following the `path`
///
/// Take the root node `parent` and the path to the node `path`
pub fn get_node<'a>(parent : &'a Node, path : &Vec<usize>) -> &'a Node {
	if path.len() == 0 {
		return parent;
	}
	get_node(&parent.child[path[0]], &path[1..].to_vec())
}

pub fn get_path<'a, 'b>(parent : &'a Node, target : &'b Node, path : &'a mut Vec<usize>, index : Option<usize>) {
	if parent == target && index.is_some() {
		path.insert(0,index.unwrap());
	}
	else {
		for i in 0..parent.child.len() {
			let size = path.len();
			get_path(&parent.child[i], target, path, Some(i));
			if size != path.len() && index.is_some() {		// A step has been added
				path.insert(0,index.unwrap());
				break;
			}
		}
	}
}
