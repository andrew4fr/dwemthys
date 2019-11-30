use crate::game::Game;
use crate::util::Contains::{DoesContain, DoesNotContain};
use crate::util::Point;
use crate::traits::RenderingComponent;
use tcod::input::KeyCode;
use tcod::input::KeyCode::{Down, Left, Right, Up};


#[derive(Copy, Clone)]
pub struct Character {
    pub pos: Point,
    pub display_char: char,
}

impl Character {
    pub fn new(x: i32, y: i32, dc: char) -> Self {
        Character {
            pos: Point { x: x, y: y },
            display_char: dc,
        }
    }

    pub fn update(&mut self, code: KeyCode, game: &Game) {
        let mut offset = Point { x: 0, y: 0 };
        match code {
            Up => {
                offset.y = -1;
            }
            Down => {
                offset.y = 1;
            }
            Left => {
                offset.x = -1;
            }
            Right => {
                offset.x = 1;
            }
            _ => {}
        }
        match game.window_bound.contains(self.pos.offset(offset)) {
            DoesContain => self.pos = self.pos.offset(offset),
            DoesNotContain => {}
        }
    }

    pub fn render(&self, mut rc: Box<dyn RenderingComponent>) {
        rc.render_object(self.pos, self.display_char);
    }
}
