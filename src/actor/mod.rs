use crate::game::Game;
use crate::util::{Bound, Point};
use crate::movement::{AggroMovementComponent, RandomMovementComponent, TcodUserMovementComponent, MovementComponent};


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

    pub fn dog(x: i32, y: i32, bound: Bound) -> Self {
        let mc = Box::new(RandomMovementComponent{window_bound: bound});
        Actor::new(x, y, 'd', mc)
    }

    pub fn cat(x: i32, y: i32, bound: Bound) -> Self {
        let mc = Box::new(RandomMovementComponent{window_bound: bound});
        Actor::new(x, y, 'c', mc)
    }

    pub fn kobold(x: i32, y: i32, bound: Bound) -> Self {
        let mc = Box::new(AggroMovementComponent{window_bound: bound});
        Actor::new(x, y, 'k', mc)
    }

    pub fn heroine(bound: Bound) -> Self {
        let point = Game::get_character_point();
        let mc = Box::new(TcodUserMovementComponent{window_bound: bound});
        Actor::new(point.x, point.y, '@', mc)
    }
}
