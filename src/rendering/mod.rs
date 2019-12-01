use tcod::{Console, RootConsole, BackgroundFlag};
use crate::util::Point;
use tcod::input::Key;

pub trait RenderingComponent {
    fn before_render_new_frame(&mut self);
    fn render_object(&mut self, pos: Point, symbol: char);
    fn after_render_new_frame(&mut self);
    fn wait_for_keypress(&mut self) -> Key;
}

pub struct TcodRenderingComponent {
    pub console: RootConsole
}

impl RenderingComponent for TcodRenderingComponent {
    fn before_render_new_frame(&mut self) {
        self.console.clear();
    }

    fn render_object(&mut self, position: Point, symbol: char) {
        self.console.put_char(position.x as i32, position.y as i32, symbol, BackgroundFlag::Set);
    }

    fn after_render_new_frame(&mut self) {
        self.console.flush();
    }

    fn wait_for_keypress(&mut self) -> Key {
        self.console.wait_for_keypress(true)
    }
}
