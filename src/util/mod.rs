pub enum XPointRelation {
    LeftOfPoint,
    RightOfPoint,
    OnPointX,
}

pub enum YPointRelation {
    AbovePoint,
    BelowPoint,
    OnPointY,
}

pub enum PointEquality {
    PointsEqual,
    PointsNotEqual,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn offset_x(&self, offset: i32) -> Point {
        Point {
            x: self.x + offset,
            y: self.y,
        }
    }

    pub fn offset_y(&self, offset: i32) -> Point {
        Point {
            x: self.x,
            y: self.y + offset,
        }
    }

    pub fn offset(&self, offset: Point) -> Point {
        Point {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }

    pub fn compare_x(&self, p: Point) -> XPointRelation {
        if self.x > p.x {
            XPointRelation::RightOfPoint
        } else if self.x < p.x {
            XPointRelation::LeftOfPoint
        } else {
            XPointRelation::OnPointX
        }
    }

    pub fn compare_y(&self, p: Point) -> YPointRelation {
        if self.y > p.y {
            YPointRelation::BelowPoint
        } else if self.y < p.y {
            YPointRelation::AbovePoint
        } else {
            YPointRelation::OnPointY
        }
    }

    pub fn compare(&self, p: Point) -> PointEquality {
        if self.x == p.x && self.y == p.y {
            PointEquality::PointsEqual
        } else {
            PointEquality::PointsNotEqual
        }
    }
}

pub enum Contains {
    DoesContain,
    DoesNotContain,
}

#[derive(Copy, Clone)]
pub struct Bound {
    pub min: Point,
    pub max: Point,
}

impl Bound {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) ->Self {
        Bound{
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y }
        }
    }

    pub fn contains(&self, point: Point) -> Contains {
        if point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
        {
            Contains::DoesContain
        } else {
            Contains::DoesNotContain
        }
    }
}
