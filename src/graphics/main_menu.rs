use conrod;
use graphics;
use ai;
use basic;
use rand;
use rand::Rng;

pub fn update(	windows : &conrod::backend::piston::Window,
		ids : &graphics::Ids,
		ui : &mut conrod::Ui,
		state : &mut graphics::State,
		node : &mut ai::Node,
		played_node : &mut Vec<usize>) {

	use conrod::{Widget, color, Positionable, Sizeable, Labelable, Colorable};
	use conrod::widget::{Canvas, Button, Text};

	let mut ui = ui.set_widgets();
	Canvas::new()
		.pad(40.0)
		.color(color::WHITE)
		.set(ids.canvas, &mut ui);

	for _click in Button::new()
		.align_middle_x_of(ids.canvas)
		.align_middle_y_of(ids.canvas)
		.w_h(150.0, 50.0)
		.align_label_middle()
		.label_font_size(20)
		.label("Solo")
		.set(ids.solo, &mut ui)
	{
		played_node.clear();
		let player = match rand::thread_rng().gen_range(0, 2){0 => 'O', _ => 'X'};
		if *node == (ai::Node{terrain : [[' ';3];3], child : Vec::new(), player : 'X', win : ' ', play : (0,0)}) || node.player != player {
			*node = ai::begin(&player, &basic::swap_player(&player));
		}
		*state = graphics::State::Solo(player,[[' ';3];3]);
	}
	for _click in Button::new()
		.down_from(ids.solo, 40.0)
		.w_h(150.0, 50.0)
		.align_label_middle()
		.label_font_size(20)
		.label("Multiplayer")
		.set(ids.multiplayer, &mut ui)
	{
		*state = graphics::State::Multiplayer('X', [[' ';3];3]);
	}
	Text::new("Tic Tac Toe")
		.up_from(ids.solo, 40.0)
		.w_h(150.0, 50.0)
		.font_size(20)
		.align_text_middle()
		.set(ids.text, &mut ui);
}
