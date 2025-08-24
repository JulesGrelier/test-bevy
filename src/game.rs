use rand::Rng;

use crate::params::*;
use crate::square::{Way, Square};

pub struct Game {
    squares : Vec<Square>,
    usable_indices : Vec<usize>,
}

impl Game {
    pub fn new() -> Self {
        let mut squares = Vec::with_capacity(NB_SQUARE as usize);
        let mut usable_indices = Vec::with_capacity(NB_SQUARE as usize);

        for id in 0..NB_SQUARE as usize {
            let f32_id = id as f32;

            let x = (SIZE_SQUARE*f32_id)%WINDOW_WIDTH ;
            let y = SIZE_SQUARE*(f32_id/NB_SQUARE_H).floor();

            let on_bottom_border = (NB_SQUARE - f32_id) <= (NB_SQUARE_H);
            let on_right_border = (f32_id % NB_SQUARE_H) == (NB_SQUARE_H-1.0);

            //println!("{}) bottom = {} et rigth = {}", id, on_bottom_border, on_right_border);

            squares.push(Square::new(x, y, id, on_bottom_border, on_right_border));
            usable_indices.push(id);
        }

        Self { squares, usable_indices }
    }

    pub fn draw_labyrinth(&self) {
        for square in &self.squares {
            square.draw();
        }
    }

    pub fn debug_labyrinth(&self) {
        for square in &self.squares {
            square.draw_debug();
        }
    }

    pub fn return_usable_square(&mut self) -> Square {
        println!("-----------------");
        loop {
            let random_nb = rand::rng().random_range(0..self.usable_indices.len());
            let random_case = self.usable_indices[random_nb];
            let way = self.squares[random_case].return_walls_to_destroy(self.return_bottom_and_right_id(random_case));

            match way {
                Way::Nothing => {
                    println!("Index {} supprimé", random_case);
                    self.remove_square_from_usable_indices(random_case)
                },
                _ => {
                    println!("Index {} choisi", random_case);
                    return self.squares[random_case]
                }
            }
        }
    }

    fn return_bottom_and_right_id(&self, index : usize) -> (Option<usize>, Option<usize>) {
        let bottom_id = match self.squares.get(index+NB_SQUARE_H as usize) {
            Some(square) => Some(square.current_id),
            None => None
        };
        
        let right_id = match self.squares.get(index+1) {
            Some(square) => Some(square.current_id),
            None => None
        };

        println!("Bottom : {:?} ------- Right : {:?}", bottom_id, right_id);
        (bottom_id, right_id)
    }

    fn remove_square_from_usable_indices(&mut self, index : usize) {
        let index_to_remove = self.usable_indices.iter().position(|&x| x == index).unwrap();
        self.usable_indices.remove(index_to_remove);
    }

    pub fn break_wall_and_return_current_and_neighbor_id(&mut self, current_square : Square) -> (usize, usize) {
        let neighbor_id = match current_square.return_walls_to_destroy(self.return_bottom_and_right_id(current_square.index))  {
            Way::Right => {
                self.squares[current_square.index].has_right_wall = false;
                self.squares[current_square.index + 1].current_id
            },
            Way::Bottom => {
                self.squares[current_square.index].has_bottom_wall = false;
                self.squares[current_square.index + NB_SQUARE_H as usize].current_id
            }
            Way::Nothing => { panic!("") }
        };

        (current_square.current_id, neighbor_id)
    }

    pub fn remplace_old_by_new_id(&mut self, replacing_id : usize, replaced_id : usize) {
        for square in &mut self.squares {
            if square.current_id == replaced_id {
                square.current_id = replacing_id;
            }
        }
    }
}