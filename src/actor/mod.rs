use crate::game::Game;
use crate::util::Point;
use crate::movement::MovementComponent;


pub struct Actor {
    pub pos: Point,
    pub display_char: char,
    pub mc: Box<dyn MovementComponent>,
}

impl Actor {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<dyn MovementComponent>) -> Self {
        Actor {
            pos: Point { x: x, y: y },
            display_char: dc,
            mc: mc,
        }
    }

    pub fn update(&mut self) {
        self.pos = self.mc.update(self.pos);
    }

    pub fn render(&self, game: &mut Game) {
        game.rc.render_object(self.pos, self.display_char);
    }
}
