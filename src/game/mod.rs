use crate::util::{Bound, Point};
use crate::traits::{Updates, RenderingComponent};
use crate::rendering::TcodRenderingComponent;
use crate::character::Character;
use tcod::input::{KeyCode, Key};

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
        let rc: Box<TcodRenderingComponent> = Box::new(RenderingComponent::new(bound));
        Game {
            exit: false,
            window_bound: bound,
            rc: rc
        }
    }

    pub fn render(&mut self, npcs: &Vec<Box<dyn Updates>>, c: Character) {
        self.rc.before_render_new_frame();
        npcs.iter().for_each(|e| e.render(self.rc));
        c.render(&mut self.rc);
        self.rc.after_render_new_frame();
    }

    pub fn update(&mut self, npcs: &mut Vec<Box<dyn Updates>>, c: &mut Character, code: KeyCode) {
        c.update(code, self);
        npcs.iter_mut().for_each(|e| e.update(self));
    }

    pub fn wait_for_keypress(&mut self) -> Key {
        self.rc.wait_for_keypress()
    }
}