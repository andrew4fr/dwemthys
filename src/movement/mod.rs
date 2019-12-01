use crate::game::Game;
use crate::util::Contains::{DoesContain, DoesNotContain};
use crate::util::{Bound, Point};
use rand::prelude::*;
use tcod::input::KeyCode::{Down, Left, Right, Up};

pub trait MovementComponent {
    fn update(&self, p: Point) -> Point;
}

pub struct RandomMovementComponent {
    pub window_bound: Bound,
}

pub struct TcodUserMovementComponent {
    pub window_bound: Bound,
}

impl MovementComponent for RandomMovementComponent {
    fn update(&self, p: Point) -> Point {
        let mut rng = rand::thread_rng();
        let mut offset = Point { x: p.x, y: p.y };

        let offset_x = rng.gen_range(0, 3i32) - 1;
        match self.window_bound.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => {
                return p;
            }
        }

        let offset_y = rng.gen_range(0, 3i32) - 1;
        match self.window_bound.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => {
                return p;
            }
        }

        offset
    }
}

impl MovementComponent for TcodUserMovementComponent {
    fn update(&self, p: Point) -> Point {
        let mut offset = Point { x: p.x, y: p.y };
        offset = match Game::get_last_keypress() {
            Some(key) => match key.code {
                Up => offset.offset_y(-1),
                Down => offset.offset_y(1),
                Left => offset.offset_x(-1),
                Right => offset.offset_x(1),
                _ => offset,
            },
            None => offset,
        };

        match self.window_bound.contains(offset) {
            DoesContain => offset,
            DoesNotContain => p,
        }
    }
}
