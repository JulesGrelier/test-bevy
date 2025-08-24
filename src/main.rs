use std::time::Duration;

use macroquad::prelude::*;
mod square;
mod game;
mod params;
use game::Game;
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
        game.debug_labyrinth();
        draw_fps();

        let current_square = game.return_usable_square();
        let old_and_new_id = game.break_wall_and_return_current_and_neighbor_id(current_square);
        game.remplace_old_by_new_id(old_and_new_id.0, old_and_new_id.1);

        std::thread::sleep(Duration::from_millis(100));
        
        next_frame().await
    }
}