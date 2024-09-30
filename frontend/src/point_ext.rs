use egui::{Pos2, Rect, Vec2};

use core::Point;

const CELL_SIZE: f32 = 10.;

pub trait PointExt {
    fn to_pos(self) -> Pos2;
    fn rect(self) -> Rect;
}

impl PointExt for Point {
    fn to_pos(self) -> Pos2 {
        Pos2::new(self.x as f32 * CELL_SIZE, self.y as f32 * CELL_SIZE)
    }

    fn rect(self) -> Rect {
        Rect::from_min_size(self.to_pos(), Vec2::splat(CELL_SIZE))
    }
}
