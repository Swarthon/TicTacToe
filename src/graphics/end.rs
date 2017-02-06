use conrod;
use graphics;
use std::process;

pub fn update(window : &conrod::backend::piston::Window, ids : &graphics::Ids, ui : &mut conrod::Ui, state : &mut graphics::State, winner : char){
	use conrod::{Widget, color, Positionable, Sizeable, Colorable, Labelable};
	use conrod::widget::{Canvas, Text, Button};
	use conrod::backend::piston::window::Size;

	let size : Size = window.window.window.get_inner_size_pixels().unwrap_or((0, 0)).into();

	let mut ui = ui.set_widgets();
	Canvas::new()
		.pad(40.0)
		.color(color::WHITE)
		.set(ids.canvas, &mut ui);

	let text = match winner {
		'X'		=> "Le Joueur X a gagné",
		'O'		=> "Le Joueur O a gagné",
		_		=> "Match nul",
	};

	Text::new(text)
		.y(size.height as f64 * 1.0 / 4.0)
		.mid_top_with_margin_on(ids.canvas, (size.height / 4) as f64)
		.w_h(150.0, 50.0)
		.font_size(20)
		.align_text_middle()
		.set(ids.text, &mut ui);

	for _click in Button::new()
		.down_from(ids.text, 40.0)
		.w_h(150.0, 50.0)
		.align_label_middle()
		.label_font_size(20)
		.label("Rejouer")
		.set(ids.replay, &mut ui)
	{
		*state = graphics::State::MainMenu;
	}
	for _click in Button::new()
		.down_from(ids.replay, 40.0)
		.w_h(150.0, 50.0)
		.align_label_middle()
		.label_font_size(20)
		.label("Quitter")
		.set(ids.quit, &mut ui)
	{
		process::exit(0);
	}
}
