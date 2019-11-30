use crate::game::Game;
use crate::traits::Updates;
use crate::util::Contains::{DoesContain, DoesNotContain};
use crate::util::Point;
use rand::prelude::*;

pub struct NPC {
    pub pos: Point,
    pub display_char: char,
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char) -> Self {
        NPC {
            pos: Point { x: x, y: y },
            display_char: dc,
        }
    }
}

impl Updates for NPC {
    fn update(&mut self, game: &Game) {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(0, 3i32) - 1;
        match game.window_bound.contains(self.pos.offset_x(offset_x)) {
            DoesContain => self.pos = self.pos.offset_x(offset_x),
            DoesNotContain => {}
        }

        let offset_y = rng.gen_range(0, 3i32) - 1;
        match game.window_bound.contains(self.pos.offset_y(offset_y)) {
            DoesContain => self.pos = self.pos.offset_y(offset_y),
            DoesNotContain => {}
        }
    }

    fn render(&self, game: &mut Game) {
        game.rc.render_object(self.pos, self.display_char);
    }
}
