use std::collections::HashSet;

use crate::Point;

pub struct Fruits {
    pub data: HashSet<Point>,
    width: u32,
    height: u32,
}

impl Fruits {
    pub fn new(fruits_count: u32, width: u32, height: u32) -> Self {
        let fruits = (0..fruits_count).map(|_| random_fruit_in(width, height));

        Self {
            data: fruits.collect(),
            width,
            height,
        }
    }

    pub fn try_eat(&mut self, point: Point) -> bool {
        self.data
            .take(&point)
            .inspect(|_| {
                self.data.insert(random_fruit_in(self.width, self.height));
            })
            .is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = Point> + '_ {
        self.data.iter().copied()
    }
}

fn random_fruit_in(width: u32, height: u32) -> Point {
    let x = (rand::random::<f32>() * (width - 1) as f32).floor() as u32;
    let y = (rand::random::<f32>() * (height - 1) as f32).floor() as u32;

    Point { x, y }
}
