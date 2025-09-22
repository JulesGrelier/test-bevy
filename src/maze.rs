use std::vec;

use crate::{params::*, square};
use crate::square::{Square, Way};

use rand::Rng;

pub struct Maze {
    pub squares : Vec<Vec<Square>>,
    usable_indices : Vec<usize>
} 

impl Maze {
    
    pub fn new () -> Self {

        let mut squares = vec![vec![]];
        let mut usable_indices = Vec::new();

        for row in 0..NB_SQUARE_V {
            squares.push(Vec::new());
            for column in 0..NB_SQUARE_H {
                let id = NB_SQUARE_H * row + column;

                squares[row].push(Square::new(row, column, id));
                usable_indices.push(id);
            }
        }

        Self { squares, usable_indices }
    }

    pub fn draw_maze(&self) {
        for row_squares in &self.squares {
            for square in row_squares {
                square.draw();
            }
        }
    }

    pub fn debug_maze(&self) {
        for row_squares in &self.squares {
            for square in row_squares {
                square.debug();
            }
        }
    }

    pub fn find_square_randomly(&self) -> Option<&Square> {

        if self.usable_indices.len() == 0 {
            return None;
        }

        let usable_index = rand::rng().random_range(0..self.usable_indices.len());
        let index = self.usable_indices[usable_index];

        let (row, column) = convert_index_to_row_and_column(index, NB_SQUARE_H);

        Some(&self.squares[row][column])
    }


    ///Doesn't check if out of range or valide way
    pub fn return_neighbor(&self, current_square : &Square, way : Way) -> &Square {

        let row = current_square.row;
        let column = current_square.column;

        match way {
            Way::Bottom => &self.squares[row+1][column],
            Way::Right => &self.squares[row][column+1],
            _ => panic!("Function return_neighbor shouldn't receive others ways")
        }
    }


    pub fn define_valide_way(&self, square : &Square) -> Way {

        let (mut bottom_acces, mut right_acces) = square.define_accesses();

        if !bottom_acces & !right_acces {
            return Way::Nothing;
        }

        if bottom_acces {
            let bottom_neighbor = self.return_neighbor(square, Way::Bottom);
            bottom_acces = square.id != bottom_neighbor.id;
        }

        if right_acces {
            let right_neighbor = self.return_neighbor(square, Way::Right);
            right_acces = square.id != right_neighbor.id;
        }

        match (bottom_acces, right_acces) {
            (true, true) => if rand::random_bool(0.5) { Way::Bottom } else { Way::Right },
            (true, false) => Way::Bottom,
            (false, true) => Way::Right,
            (false, false) => Way::Nothing,
        }
    }


    pub fn remplace_old_by_new_id(&mut self, replacing_id : usize, replaced_id : usize) {
        for row_squares in &mut self.squares {
            for square in row_squares {
                if square.id == replaced_id {
                    square.id = replacing_id;
                }
            }
        }
    }

    pub fn remove_from_usable_indices(&mut self, row : usize, column : usize) {
        let index = convert_row_and_column_to_index(row, column, NB_SQUARE_H);

        let useless_index = self.usable_indices.iter().position(|x| *x == index).unwrap();
        self.usable_indices.remove(useless_index);
    }

}


fn convert_index_to_row_and_column(index : usize, nb_square_h : usize) -> (usize, usize) {
    let row = round::round_down(index as f64 / nb_square_h as f64, 0) as usize;
    let column = index - (row*nb_square_h);

    (row, column)
}

fn convert_row_and_column_to_index(row : usize, column : usize, nb_square_h : usize) -> usize {
    row*nb_square_h + column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_convert_index_to_row_and_column() {
        assert_eq!(convert_index_to_row_and_column(10, 300), (0, 10));
        assert_eq!(convert_index_to_row_and_column(20, 15), (1, 5));
    }

    #[test]
    fn check_convert_row_and_column_to_index() {
        assert_eq!(convert_row_and_column_to_index(1, 1, 20), 21);
        assert_eq!(convert_row_and_column_to_index(0, 8, 13), 8);
        assert_eq!(convert_row_and_column_to_index(1, 0, 42), 42);
    }

}