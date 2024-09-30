use std::collections::HashMap;
use std::iter;
use std::ops::{Index, IndexMut};

use ecolor::Color32;
use macros::id;
use serde::{Deserialize, Serialize};

use crate::Point;

#[derive(Default)]
pub struct Snakes {
    pub data: HashMap<SnakeID, Snake>,

    ids_cointer: u32,
}

impl Snakes {
    pub fn add(&mut self, snake: Snake) -> SnakeID {
        let id = SnakeID::new(self.ids_cointer);

        self.ids_cointer += 1;

        self.data.insert(id, snake);

        id
    }

    pub fn iter(&self) -> impl Iterator<Item = (SnakeID, &Snake)> {
        self.data.iter().map(|(&id, snake)| (id, snake))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (SnakeID, &mut Snake)> {
        self.data.iter_mut().map(|(&id, snake)| (id, snake))
    }

    pub fn snakes(&self) -> impl Iterator<Item = &Snake> {
        self.data.values()
    }

    pub fn snakes_mut(&mut self) -> impl Iterator<Item = &mut Snake> {
        self.data.values_mut()
    }

    pub fn heads(&self) -> impl Iterator<Item = (SnakeID, Point)> + '_ {
        self.iter().map(|(id, snake)| (id, snake.head))
    }

    pub fn cells(&self) -> impl Iterator<Item = Point> + '_ {
        self.snakes().map(|snake| snake.body()).flatten()
    }

    pub fn remove_snake(&mut self, snake: SnakeID) -> Snake {
        self.data.remove(&snake).unwrap()
    }
}

impl Index<SnakeID> for Snakes {
    type Output = Snake;

    fn index(&self, id: SnakeID) -> &Self::Output {
        self.data.get(&id).unwrap()
    }
}

impl IndexMut<SnakeID> for Snakes {
    fn index_mut(&mut self, id: SnakeID) -> &mut Self::Output {
        self.data.get_mut(&id).unwrap()
    }
}

#[id]
pub struct SnakeID;

#[derive(Serialize, Deserialize, Clone)]
pub struct Snake {
    pub head: Point,
    pub tail: Vec<Point>,
    pub color: Color32,
}

impl Snake {
    pub fn from_dir_len(head: Point, dir: Direction, len: usize, color: Color32) -> Self {
        let mut tail = Vec::with_capacity(len);

        for _ in 0..len {
            let &last = tail.last().unwrap_or(&head);

            tail.push(last + dir.opposite());
        }

        Self { head, tail, color }
    }

    pub fn grow(&mut self) {
        self.tail.push(self.last())
    }

    pub fn body(&self) -> impl Iterator<Item = Point> + '_ {
        iter::once(self.head()).chain(self.tail())
    }

    pub fn head(&self) -> Point {
        self.head
    }

    pub fn tail(&self) -> impl Iterator<Item = Point> + '_ {
        self.tail.iter().copied()
    }

    pub fn last(&self) -> Point {
        self.tail.last().copied().unwrap_or(self.head)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}
