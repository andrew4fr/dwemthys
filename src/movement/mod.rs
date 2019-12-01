use crate::game::Game;
use crate::util::Contains::{DoesContain, DoesNotContain};
use crate::util::{Bound, Point, XPointRelation, YPointRelation, PointEquality};
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

pub struct AggroMovementComponent {
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

impl MovementComponent for AggroMovementComponent {
    fn update(&self, p: Point) -> Point {
        let char_pos = Game::get_character_point();
        let mut offset = Point{ x: 0, y: 0 };

        match p.compare_x(char_pos) {
            XPointRelation::RightOfPoint => offset = offset.offset_x(-1),
            XPointRelation::LeftOfPoint => offset = offset.offset_x(1),
            XPointRelation::OnPointX => {},
        }

        match p.compare_y(char_pos) {
            YPointRelation::BelowPoint => offset = offset.offset_y(-1),
            YPointRelation::AbovePoint => offset = offset.offset_y(1),
            YPointRelation::OnPointY => {},
        }

        match p.offset(offset).compare(char_pos) {
            PointEquality::PointsEqual => return p,
            PointEquality::PointsNotEqual => {
                match self.window_bound.contains(p.offset(offset)) {
                    DoesContain => { return p.offset(offset); },
                    DoesNotContain => { return p; },
                }
            }
        }
    }
}