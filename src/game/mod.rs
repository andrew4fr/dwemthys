use crate::util::{Bound, Point};
use crate::rendering::{RenderingComponent, TcodRenderingComponent, WindowComponent, TcodStatWindowComponent, TcodInputWindowComponent, TcodMessageWindowComponent, TcodMapWindowComponent};
use crate::actor::Actor;
use tcod::input::Key;

static mut LAST_KEYPRESS: Option<Key> = None;
static mut CHAR_LOCATION: Point = Point{ x: 40, y: 25 };

pub struct Game {
    pub exit: bool,
    pub window_bound: Bound,
    pub rc: Box<dyn RenderingComponent>,
    pub stat_window: Box<dyn WindowComponent>,
    pub input_window: Box<dyn WindowComponent>,
    pub message_window: Box<dyn WindowComponent>,
    pub map_window: Box<dyn WindowComponent>,
}

impl Game {
    pub fn new() -> Self {
        let total_bound = Bound::new(0, 0, 99, 61);
        let stat_bound = Bound::new(79, 0, 99, 49);
        let input_bound = Bound::new(0, 50, 99, 52);
        let message_bound = Bound::new(0, 53, 99, 61);
        let map_bound = Bound::new(0, 0, 78, 49);

        let rc = Box::new(TcodRenderingComponent::new(total_bound));
        let stat_w = Box::new(TcodStatWindowComponent::new(stat_bound));
        let input_w = Box::new(TcodInputWindowComponent::new(input_bound));
        let message_w = Box::new(TcodMessageWindowComponent::new(message_bound));
        let map_w = Box::new(TcodMapWindowComponent::new(map_bound));

        Game {
            exit: false,
            window_bound: total_bound,
            rc: rc,
            stat_window: stat_w,
            input_window: input_w,
            message_window: message_w,
            map_window: map_w,
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<Actor>>, a: &Actor) {
        self.rc.before_render_new_frame();
        self.rc.attach_window(&mut self.stat_window);
        self.rc.attach_window(&mut self.input_window);
        self.rc.attach_window(&mut self.message_window);
        self.rc.attach_window(&mut self.map_window);
        npcs.iter().for_each(|e| e.render(self));
        a.render(self);
        self.rc.after_render_new_frame();
    }

    pub fn update(&self, npcs: &mut Vec<Box<Actor>>, a: &mut Actor) {
        a.update();
        Game::set_character_point(a.pos);
        npcs.iter_mut().for_each(|e| e.update());
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        let key = self.rc.wait_for_keypress();
        match key.printable {
            '/' => self.input_window.buffer_message("Wich direction would you like to attack with your heoric sword? [Press an arrow key]"),
            _ => self.input_window.flush_buffer(),
        }
        Game::set_last_keypress(key);

        key
    }

    pub fn get_last_keypress() -> Option<Key> {
        unsafe { LAST_KEYPRESS }
    }

    pub fn set_last_keypress(key: Key) {
        unsafe { LAST_KEYPRESS = Some(key); }
    }

    pub fn get_character_point() -> Point {
        unsafe { CHAR_LOCATION }
    }

    pub fn set_character_point(p: Point) {
        unsafe { CHAR_LOCATION = p; }
    }
}
