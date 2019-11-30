use crate::util::{Point, Bound};
use rand::prelude::*;
use crate::util::Contains::{DoesContain, DoesNotContain};

pub trait MovementComponent {
    fn update(&self, p: Point) -> Point;
}

pub struct RandomMovementComponent {
    pub window_bound: Bound,
}

impl MovementComponent for RandomMovementComponent {
    fn update(&self, p: Point) -> Point {
        let mut rng = rand::thread_rng();
        let mut offset = Point{ x: p.x, y: p.y };

        let offset_x = rng.gen_range(0, 3i32) - 1;
        match self.window_bound.contains(offset.offset_x(offset_x)) {
            DoesContain => offset = offset.offset_x(offset_x),
            DoesNotContain => { return p; }
        }

        let offset_y = rng.gen_range(0, 3i32) - 1;
        match self.window_bound.contains(offset.offset_y(offset_y)) {
            DoesContain => offset = offset.offset_y(offset_y),
            DoesNotContain => { return p; }
        }

        offset
    }
}