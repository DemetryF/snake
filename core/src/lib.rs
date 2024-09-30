mod events;
mod fruits;
mod snakes;
mod world;

use events::Events;

pub use snakes::{Direction, Snake, SnakeID};
pub use world::{Point, World};

pub struct GameState {
    pub world: World,
    pub events: Events,
}

impl GameState {
    pub fn new(world: World) -> Self {
        Self {
            world,
            events: Default::default(),
        }
    }

    pub fn update(&mut self) {
        self.world.update(&mut self.events);
    }

    pub fn change_dir(&mut self, snake: SnakeID, new_dir: Direction) {
        self.world.directions.insert(snake, new_dir);
    }

    pub fn add_snake(&mut self, snake: Snake, dir: Direction) -> SnakeID {
        let id = self.world.snakes.add(snake);

        self.world.directions.insert(id, dir);

        id
    }

    pub fn cells(&self) -> impl Iterator<Item = Point> + '_ {
        self.world.snakes.cells()
    }

    pub fn fruits(&self) -> impl Iterator<Item = Point> + '_ {
        self.world.fruits.iter()
    }
}
