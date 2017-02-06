use conrod;
use find_folder;

use ai;

mod main_menu;
mod solo;
mod multiplayer;
mod end;

#[derive(PartialEq, Eq)]
pub enum State {
	MainMenu,
	Solo(char,[[char;3];3]),
	Multiplayer(char,[[char;3];3], ai::Node),
	End(char),
}

widget_ids! {
	pub struct Ids {
        	canvas,
		solo,
		multiplayer,
		text,
		replay,
		quit,
		case[],
	}
}

pub fn main() {
	use conrod::backend::piston::{self, Window, WindowEvents, OpenGL};
	use conrod::backend::piston::event::UpdateEvent;

	let mut window: Window = piston::window::WindowSettings::new("Morpion | Tic Tac Toe", [512, 512])
        	.opengl(OpenGL::V3_2)
        	.build()
        	.unwrap();

	let mut events = WindowEvents::new();

	let mut text_texture_cache = piston::window::GlyphCache::new(&mut window, 512, 512);
	let image_map = conrod::image::Map::new();

	let mut ui = conrod::UiBuilder::new([512.0 as f64, 512.0 as f64]).build();
	let assets = find_folder::Search::KidsThenParents(3, 5).for_folder("assets").unwrap();
	let font_path = assets.join("fonts/NotoSans/NotoSans-Bold.ttf");
	ui.fonts.insert_from_file(font_path).unwrap();
	let mut ids = Ids::new(ui.widget_id_generator());
	ids.case.resize(10, &mut ui.widget_id_generator());

	let mut state = State::MainMenu;

	while let Some(event) = window.next_event(&mut events) {
		if let Some(e) = piston::window::convert_event(event.clone(), &window) {
			ui.handle_event(e);
        	}

		event.update(|_| {
			let mut s = State::MainMenu;
			match state {
				State::MainMenu					=> main_menu::update(&window, &ids, &mut ui, &mut state),
				State::Solo(player, terrain)			=> solo::update(&window, &ids, &mut ui, &mut state, player, terrain),
				State::Multiplayer(player,terrain, ref node)	=> s = multiplayer::update(&window, &ids, &mut ui, player, terrain, node),
				State::End(winner)				=> end::update(&window, &ids, &mut ui, &mut state, winner),
			}
			state = s;
		});

		window.draw_2d(&event, |c, g| {
			if let Some(primitives) = ui.draw_if_changed() {
				fn texture_from_image<T>(img: &T) -> &T {img};
                		piston::window::draw(
					c,
					g,
					primitives,
                        		&mut text_texture_cache,
                                	&image_map,
                        		texture_from_image);
			}
		});
	}
}
