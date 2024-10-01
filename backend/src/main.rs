use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use std::time::{Duration, Instant};

use random_color::options::Luminosity;
use random_color::RandomColor;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::time::sleep;

use core::{Direction, GameState, Point, Snake, SnakeID, World};

use protocol::{JoinPacket, StatePacketRef};

#[tokio::main]
async fn main() {
    let game_state = GameState::new(World::new(50, 50, 10));
    let game_state = Arc::new(RwLock::new(game_state));

    let connections = Connections::default();

    let update_task = tokio::spawn({
        let game_state = Arc::clone(&game_state);
        let connections = connections.clone();

        async move {
            const FPS: f32 = 6.;

            let mut buffer = Vec::new();
            let mut last_iter_dur = 1. / FPS;

            loop {
                sleep(Duration::from_secs_f32(f32::max(
                    1. / FPS - last_iter_dur,
                    0.,
                )))
                .await;

                let iter_start = Instant::now();

                {
                    game_state.write().await.update();
                }

                {
                    buffer.clear();

                    let game_state = game_state.read().await;

                    let packet = StatePacketRef {
                        snakes: &game_state.world.snakes.data,
                        fruits: &game_state.world.fruits.data,
                    };

                    bincode::serialize_into(&mut buffer, &packet).unwrap();

                    connections.send_all(&buffer).await;
                }

                last_iter_dur = iter_start.elapsed().as_secs_f32();
            }
        }
    });

    let listenter = TcpListener::bind("192.168.0.11:1984").await.unwrap();

    let accept_task = tokio::spawn({
        let connections = connections.clone();

        let mut random_color = RandomColor {
            luminosity: Some(Luminosity::Dark),
            ..Default::default()
        };

        async move {
            loop {
                let (socket, _) = listenter.accept().await.unwrap();

                let (mut read_socket, mut write_socket) = socket.into_split();

                let game_state = Arc::clone(&game_state);

                let id = {
                    let mut game_state = game_state.write().await;

                    let head = Point::new(game_state.world.width / 2, game_state.world.height / 2);
                    let dir = Direction::Left;

                    game_state.add_snake(
                        Snake::from_dir_len(head, dir, 4, random_color.to_color32()),
                        dir,
                    )
                };

                {
                    let game_state = game_state.read().await;

                    let packet = JoinPacket {
                        width: game_state.world.width,
                        height: game_state.world.height,
                        id,
                    };

                    let mut buffer = Vec::new();

                    bincode::serialize_into(&mut buffer, &packet).unwrap();

                    write_socket.write(&buffer).await.unwrap();
                }

                let connections = connections.clone();

                connections.add(id, write_socket).await;

                tokio::spawn(async move {
                    let mut buffer = Vec::new();

                    loop {
                        buffer.clear();

                        for _ in 0..4 {
                            let Ok(read_u8) = read_socket.read_u8().await else {
                                connections.remove(id).await;
                                game_state.write().await.world.snakes.data.remove(&id);

                                return;
                            };

                            buffer.push(read_u8);
                        }

                        let direction = bincode::deserialize_from(buffer.as_slice()).unwrap();

                        let mut write = game_state.write().await;

                        write.change_dir(id, direction);
                    }
                });
            }
        }
    });

    let _ = tokio::join!(update_task, accept_task);
}

#[derive(Clone, Default)]
pub struct Connections {
    data: Arc<RwLock<HashMap<SnakeID, Connection>>>,
}

impl Connections {
    pub async fn add(&self, id: SnakeID, write_socket: OwnedWriteHalf) {
        self.data
            .write()
            .await
            .insert(id, Connection { write_socket });
    }

    pub async fn remove(&self, id: SnakeID) {
        self.data.write().await.remove(&id);
    }

    pub async fn send_all(&self, data: &[u8]) {
        let mut connections = self.data.write().await;

        for connection in connections.values_mut() {
            let _ = connection.send(data).await;
        }
    }
}

pub struct Connection {
    pub write_socket: OwnedWriteHalf,
}

impl Connection {
    pub async fn send(&mut self, data: &[u8]) -> io::Result<()> {
        self.write_socket.write_all(data).await
    }
}
