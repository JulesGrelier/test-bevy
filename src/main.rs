//use std::time::Duration;

use macroquad::prelude::*;
mod square;
mod game;
mod params;
use game::Game;
use ::rand::seq::index;
use crate::params::*;

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Labyrinth Simulation"),
        window_width : WINDOW_WIDTH as i32,
        window_height : WINDOW_HEIGHT as i32,
        window_resizable : false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    if 1080.0 % SIZE_SQUARE != 0.0 && 720.0 % SIZE_SQUARE != 0.0 {
        panic!("Les colonnes et lignes de carrés doivent tombés juste");
    }

    let mut game = Game::new();

    loop {

        clear_background(BLACK);
        game.draw_labyrinth();
        draw_fps();

        for _ in 0..200 {
            let a = game.make_one_cycle();

            match a {
                game::Situation::UselessIndex(index) => game.remove_square_from_usable_indices(index),
                _ => {}
            }
        }

        //std::thread::sleep(Duration::from_millis(10));
        
        next_frame().await
    }
}