use crate::game::Game;
use crate::traits::Updates;
use crate::util::Point;
use crate::movement::MovementComponent;

pub struct NPC {
    pub pos: Point,
    pub display_char: char,
    mc: Box<dyn MovementComponent>,
}

impl NPC {
    pub fn new(x: i32, y: i32, dc: char, mc: Box<dyn MovementComponent>) -> Self {
        NPC {
            pos: Point { x: x, y: y },
            display_char: dc,
            mc: mc,
        }
    }
}

impl Updates for NPC {
    fn update(&mut self) {
        self.pos = self.mc.update(self.pos);
    }

    fn render(&self, game: &mut Game) {
        game.rc.render_object(self.pos, self.display_char);
    }
}
