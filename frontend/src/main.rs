mod painter;
mod point_ext;

use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread;

use eframe::NativeOptions;
use egui::emath::TSTransform;
use egui::{Color32, Frame, Key, Margin, Rect, Sense};

use core::{Direction, Point};
use protocol::{JoinPacket, StatePacket};

use painter::Painter;
use point_ext::PointExt;

fn main() {
    let state = Arc::new(RwLock::new(StatePacket::default()));
    let socket = TcpStream::connect("192.168.0.11:1984").unwrap();

    let join: JoinPacket = bincode::deserialize_from(&socket).unwrap();

    thread::spawn({
        let state = Arc::clone(&state);
        let socket = socket.try_clone().unwrap();

        let mut buffer = Vec::new();

        move || loop {
            let mut socket = socket.try_clone().unwrap();

            let mut package_size = [0; 8];

            socket.read(&mut package_size).unwrap();

            let package_size = u64::from_be_bytes(package_size) as usize;

            buffer.resize(package_size, 0);
            socket.read_exact(&mut buffer).unwrap();

            let mut state = state.write().unwrap();

            *state = bincode::deserialize_from(buffer.as_slice()).unwrap();
        }
    });

    let transform = TSTransform::IDENTITY;

    eframe::run_simple_native("snake", NativeOptions::default(), move |ctx, _| {
        ctx.request_repaint();

        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::ZERO,
                outer_margin: Margin::ZERO,
                ..Default::default()
            })
            .show(ctx, |ui| {
                ctx.input(|state| {
                    if state.key_down(Key::W) {
                        change_dir(&socket, Direction::Up);
                    } else if state.key_down(Key::A) {
                        change_dir(&socket, Direction::Left);
                    } else if state.key_down(Key::S) {
                        change_dir(&socket, Direction::Down);
                    } else if state.key_down(Key::D) {
                        change_dir(&socket, Direction::Right);
                    }
                });

                let (_, painter) =
                    ui.allocate_painter(ui.available_size(), Sense::click_and_drag());

                let painter = Painter {
                    raw: painter,
                    transform,
                };

                const SNAKE_COLOR: Color32 = Color32::from_rgb(228, 228, 30);
                const GRASS_COLOR: Color32 = Color32::from_rgb(30, 228, 40);
                const FRUIT_COLOR: Color32 = Color32::from_rgb(228, 30, 30);

                let state = state.read().unwrap();

                let rect = Rect::from_min_max(
                    Point::new(1, 1).to_pos(),
                    Point::new(join.width, join.height).to_pos(),
                );

                painter.rect(rect, GRASS_COLOR);

                for snake in state.snakes.values() {
                    for point in snake.body() {
                        painter.rect(point.rect(), SNAKE_COLOR);
                    }
                }

                for &fruit in state.fruits.iter() {
                    painter.rect(fruit.rect(), FRUIT_COLOR);
                }
            });
    })
    .unwrap();
}

pub fn change_dir(socket: &TcpStream, dir: Direction) {
    bincode::serialize_into(socket, &dir).unwrap();
}
