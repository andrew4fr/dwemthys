use crate::game::Game;
use crate::util::Point;
use tcod::input::Key;

pub trait Updates {
    fn update(&mut self, game: &Game);
    fn render(&self, game: &mut Game);
}

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, pos: Point, symbol: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
}
