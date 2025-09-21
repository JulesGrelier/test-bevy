//use std::time::Duration;

use std::time::Duration;

use macroquad::prelude::*;
mod square;
mod game;
mod maze;
mod params;
use maze::Maze;

use crate::{params::*, square::Way};

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

    if WINDOW_WIDTH % SIZE_SQUARE != 0.0 && WINDOW_HEIGHT % SIZE_SQUARE != 0.0 {
        panic!("Les colonnes et lignes de carrés doivent tombés juste");
    }

    let mut maze = Maze::new();

    println!("c est le debut");

    loop {

        clear_background(BLACK);
        println!("c est le A");
        maze.draw_maze();
        println!("c est le Z");
        draw_fps();

        for _ in 0..400 {

            let (mut row, mut column) = (0, 0);
            let mut neighbor_id = 0;
            let mut way = Way::Nothing;

            //println!("AAAAAAA");

            {
                //println!("BBBBBBB");
                let square = match maze.find_square_randomly() {
                    Some(square) => square,
                    None => continue,
                };

                //println!("CCCCCC");
                (way, neighbor_id) = maze.define_if_valide(square);


                row = square.row_index;
                column = square.column_index;

                //println!("DDDDDDD");
            }

            if way == Way::Right || way == Way::Bottom {
                maze.squares[row][column].break_wall(way);
                maze.remplace_old_by_new_id(maze.squares[row][column].id, neighbor_id);
                continue;
            }

            maze.remove_from_usable_indices(row, column);
        }

        //std::thread::sleep(Duration::from_secs(1));
        
        next_frame().await
    }
}