use crate::util::{Bound, Point};
use crate::rendering::{RenderingComponent, TcodRenderingComponent};
use crate::actor::Actor;
use tcod::input::Key;
use tcod::RootConsole;

static mut LAST_KEYPRESS: Option<Key> = None;

pub struct Game {
    pub exit: bool,
    pub window_bound: Bound,
    pub rc: Box<dyn RenderingComponent>,
}

impl Game {
    pub fn new() -> Self {
        let bound = Bound {
            min: Point { x: 0, y: 0 },
            max: Point { x: 79, y: 49 }
        };
        let con = RootConsole::initializer()
            .size(bound.max.x + 1, bound.max.y + 1)
            .title("Rogalik")
            .init();

        let rc = Box::new(TcodRenderingComponent{console: con});
        Game {
            exit: false,
            window_bound: bound,
            rc: rc
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, a: &Actor) {
        self.rc.before_render_new_frame();
        npcs.iter().for_each(|e| e.render(self));
        a.render(self);
        self.rc.after_render_new_frame();
    }

    pub fn update(&self, npcs: &mut Vec<Box<Actor>>, a: &mut Actor) {
        a.update();
        npcs.iter_mut().for_each(|e| e.update());
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let key = self.rc.wait_for_keypress();
        Game::set_last_keypress(key);

        key
    }

    pub fn get_last_keypress() -> Option<Key> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(key: Key) {
        unsafe { LAST_KEYPRESS = Some(key); }
    }
}
