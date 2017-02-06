use conrod;
use graphics;
use basic;

pub fn update(window : &conrod::backend::piston::Window, ids : &graphics::Ids, ui : &mut conrod::Ui, state : &mut graphics::State, player : char, terrain : [[char;3];3]){
	use conrod::{Widget, color, Positionable, Sizeable, Colorable, Labelable};
	use conrod::widget::{Canvas, Text, Button};
	use conrod::backend::piston::window::Size;

	let size : Size = window.window.window.get_inner_size_pixels().unwrap_or((0, 0)).into();

	let mut ui = ui.set_widgets();
	Canvas::new()
		.pad(40.0)
		.color(color::WHITE)
		.set(ids.canvas, &mut ui);

	Text::new(format!("Au tour de {} de jouer", player).as_str())
		.y(size.height as f64 * 1.0 / 4.0)
		.mid_top_with_margin_on(ids.canvas, (size.height / 10) as f64)
		.w_h(150.0, 50.0)
		.font_size(20)
		.align_text_middle()
		.set(ids.text, &mut ui);

	let o = 0.01 * ui.w_of(ids.canvas).unwrap();
	let w = (size.width as f64 - 2.0 * o) / 6.0;

	let p = 0.01 * size.height as f64;
	let q = 0.02 * size.height as f64;
	let h = (size.height as f64 - q - p) / 6.0;

	let mut terrain = terrain;
	let mut player = player;

	for x in 0..3 {
		for y in 0..3 {
			for _click in Button::new()
				.h(h)
				.w(w)
				.x(o + w * x as f64 - w as f64)
				.y(p + h * y as f64 - h * 2 as f64)
				.align_label_middle()
				.label_font_size(20)
				.label(format!("{}", terrain[y][x]).as_str())
				.set(ids.case[x*3+y], &mut ui)
			{
				let played = basic::play(&mut terrain, x, y, &mut player);
				match basic::test_win(&terrain){
					'X'	=> {println!("test"); *state = graphics::State::End('X')},
					'O'	=> *state = graphics::State::End('O'),
					'0'	=> *state = graphics::State::End(' '),
					_	=> continue,
				}
				return;
			}
		}
	}

	*state = graphics::State::Solo(player, terrain);
}
