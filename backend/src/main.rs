use std::sync::Arc;
use std::time::{Duration, Instant};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::{sync::RwLock, time::sleep};

use core::{Direction, GameState, Point, Snake, World};
use protocol::{JoinPacket, StatePacketRef};

#[tokio::main]
async fn main() {
    let game_state = GameState::new(World::new(50, 50, 10));
    let game_state = Arc::new(RwLock::new(game_state));

    let connections = Arc::new(RwLock::new(Vec::<Connection>::new()));

    let update_task = tokio::spawn({
        let game_state = Arc::clone(&game_state);
        let connections = Arc::clone(&connections);

        async move {
            const FPS: f32 = 4.;

            let mut buffer = Vec::<u8>::new();
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
                    {
                        buffer.clear();

                        let game_state = game_state.read().await;

                        let packet = StatePacketRef {
                            snakes: &game_state.world.snakes.data,
                            fruits: &game_state.world.fruits.data,
                        };

                        bincode::serialize_into(&mut buffer, &packet).unwrap();
                    }

                    let connections = Arc::clone(&connections);

                    for connection in connections.write().await.iter_mut() {
                        connection
                            .write_socket
                            .write_u64(buffer.len() as u64)
                            .await
                            .unwrap();

                        connection.write_socket.write_all(&buffer).await.unwrap();
                    }
                }

                last_iter_dur = iter_start.elapsed().as_secs_f32();
            }
        }
    });

    let listenter = TcpListener::bind("192.168.0.11:1984").await.unwrap();

    let accept_task = tokio::spawn({
        let connections = Arc::clone(&connections);

        async move {
            loop {
                let (socket, _) = listenter.accept().await.unwrap();

                let (mut read_socket, mut write_socket) = socket.into_split();

                let game_state = Arc::clone(&game_state);

                let id = {
                    let mut game_state = game_state.write().await;

                    let head = Point::new(game_state.world.width / 2, game_state.world.height / 2);
                    let dir = Direction::Left;

                    game_state.add_snake(Snake::from_dir_len(head, dir, 4), dir)
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

                {
                    connections.write().await.push(Connection { write_socket });
                }

                tokio::spawn(async move {
                    let mut buffer = Vec::new();

                    loop {
                        buffer.clear();

                        for _ in 0..4 {
                            loop {
                                if let Ok(byte) = read_socket.read_u8().await {
                                    buffer.push(byte);
                                    break;
                                }
                            }
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

pub struct Connection {
    pub write_socket: OwnedWriteHalf,
}
