use std::collections::{HashMap, HashSet};

use core::{Point, Snake, SnakeID};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default)]
pub struct StatePacket {
    pub snakes: HashMap<SnakeID, Snake>,
    pub fruits: HashSet<Point>,
}

#[derive(Serialize)]
pub struct StatePacketRef<'frame> {
    pub snakes: &'frame HashMap<SnakeID, Snake>,
    pub fruits: &'frame HashSet<Point>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct JoinPacket {
    pub width: u32,
    pub height: u32,
    pub id: SnakeID,
}
