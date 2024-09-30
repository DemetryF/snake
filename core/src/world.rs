use std::collections::HashMap;
use std::ops;

use serde::{Deserialize, Serialize};

use crate::events::Events;
use crate::fruits::Fruits;
use crate::snakes::Snakes;
use crate::{Direction, SnakeID};

pub struct World {
    pub snakes: Snakes,
    pub fruits: Fruits,

    pub directions: HashMap<SnakeID, Direction>,

    pub height: u32,
    pub width: u32,
}

impl World {
    pub fn new(height: u32, width: u32, fruits_count: u32) -> Self {
        Self {
            height,
            width,
            snakes: Snakes::default(),
            fruits: Fruits::new(fruits_count, width, height),
            directions: HashMap::default(),
        }
    }

    pub fn update(&mut self, events: &mut Events) {
        let mut hit_snakes = vec![];

        for (id, snake) in self.snakes.iter_mut() {
            for idx in (1..snake.tail.len()).rev() {
                snake.tail[idx] = snake.tail[idx - 1];
            }

            snake.tail[0] = snake.head;

            snake.head = snake.head + self.directions[&id];

            if self.fruits.try_eat(snake.head()) {
                snake.grow();
            }
        }

        for (id, head) in self.snakes.heads() {
            if self.out_of_world(head) || {
                self.snakes
                    .iter()
                    .filter(|&(other_id, _)| other_id != id)
                    .map(|(_, snake)| snake.tail())
                    .flatten()
                    .any(|point| point == head)
            } {
                hit_snakes.push(id);
            }
        }

        for id in hit_snakes {
            let snake = self.snakes.remove_snake(id);

            events.emit_hit(id, snake);
        }
    }

    fn out_of_world(&self, point: Point) -> bool {
        point.x == 0 || point.x == self.width || point.y == 0 || point.y == self.height
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Point {
        Self { x, y }
    }
}

impl ops::Add<Direction> for Point {
    type Output = Point;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => Point::new(self.x, self.y - 1),
            Direction::Down => Point::new(self.x, self.y + 1),
            Direction::Left => Point::new(self.x - 1, self.y),
            Direction::Right => Point::new(self.x + 1, self.y),
        }
    }
}
