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
            let on_right_border = f32_id+1.0 % NB_SQUARE_H == 0.0;

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

    pub fn make_one_cycle(&mut self) {

        loop {

            if self.usable_indices.len() == 0 {
                return;
            }

            let random_index = self.usable_indices[rand::rng().random_range(0..self.usable_indices.len())];
            let acceses = self.squares[random_index].return_bottom_and_right_acces();

            if !acceses.0 & !acceses.1 {
                self.remove_square_from_usable_indices(random_index);
                continue;
            }

            let id = self.squares[random_index].id;

            let bottom_neighbor = if let Some(neighbor) =  self.return_neighbor(random_index, Way::Bottom) {
                if neighbor.id != id { Some(neighbor) } else { None }
            } else { None };

            let right_neighbor = if let Some(neighbor) =  self.return_neighbor(random_index, Way::Right) {
                if neighbor.id != id { Some(neighbor) } else { None } 
            } else {  None };

            let (neighbor, way) = match (bottom_neighbor, right_neighbor) {
                (None, None) => {
                    self.remove_square_from_usable_indices(random_index);
                    continue;
                },

                (None, Some(right_neighbor)) => (right_neighbor, Way::Right),

                (Some(bottom_neighbor), None) => (bottom_neighbor, Way::Bottom),

                (Some(bottom_neighbor), Some(right_neighbor)) => 
                if rand::rng().random_bool(0.5) { (right_neighbor, Way::Right) } else { (bottom_neighbor, Way::Bottom) },
            };

            self.remplace_old_by_new_id(id, neighbor.id);
            self.squares[random_index].break_wall(way);

            return;
        }
    }

    fn return_neighbor(&self, index : usize, way : Way) -> Option<&Square> {
        match way {
            Way::Bottom => return self.squares.get(index+NB_SQUARE_H as usize),
            Way::Right => {
                if (index+1) % NB_SQUARE_H as usize == 0 { //Parentheses in (index+1) are important !!! Don't touch
                    None
                } else { self.squares.get(index+1) }
            },
            Way::Nothing => None
        }
    }

    fn remove_square_from_usable_indices(&mut self, index : usize) {
        let index_to_remove = self.usable_indices.iter().position(|&x| x == index).unwrap();
        self.usable_indices.remove(index_to_remove);
    }

    pub fn remplace_old_by_new_id(&mut self, replacing_id : usize, replaced_id : usize) {
        for square in &mut self.squares {
            if square.id == replaced_id {
                square.id = replacing_id;
            }
        }
    }
}