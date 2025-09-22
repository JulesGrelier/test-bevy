use std::time::Duration;
use macroquad::prelude::*;

mod square;
mod maze;
mod params;

use crate::{params::*, square::Way, maze::Maze};

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("Maze Simulation"),
        window_width : WINDOW_WIDTH as i32,
        window_height : WINDOW_HEIGHT as i32,
        window_resizable : false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    if WINDOW_WIDTH % SIZE_SQUARE != 0.0 && WINDOW_HEIGHT % SIZE_SQUARE != 0.0 {
        panic!("Les colonnes et lignes de carrés doivent tombés juste");
    }

    let mut maze = Maze::new();
    let mut running = true;

    loop {

        clear_background(BLACK);
        maze.draw_maze();
        draw_fps();

        for _ in 0..800 {

            let square = match maze.find_square_randomly() {
                Some(square) => square,
                None => {
                    running = false;
                    println!("Terminé");
                    break;
                },
            };

            let (row, column) = (square.row, square.column);

            let way = maze.define_valide_way(square);

            if way == Way::Nothing {
                maze.remove_from_usable_indices(row, column);
                continue;
            }

            let current_id = maze.squares[row][column].id;
            let neighbor_id = maze.return_neighbor(square, way).id;

            maze.squares[row][column].break_wall(way);
            maze.remplace_old_by_new_id(current_id, neighbor_id);
        }

        if !running {
            std::thread::sleep(Duration::from_secs(1));
        }

        next_frame().await
    }
}