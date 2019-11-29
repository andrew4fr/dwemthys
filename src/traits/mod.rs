use crate::game::Game;
use crate::util::Point;
use crate::rendering::TcodRenderingComponent;
use tcod::input::Key;
use tcod::RootConsole;

pub trait Updates {
    fn update(&mut self, game: Game);
    fn render(&self, rc: &mut TcodRenderingComponent);
}

pub trait RenderingComponent {
    fn new(con: RootConsole) -> Self;
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, pos: Point, symbol: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
}
