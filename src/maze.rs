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

        println!("AAAAAAA");
        let mut squares = vec![vec![]];
        println!("BBBBBBBB");
        let mut usable_indices = Vec::new();
        println!("CCCCCCCC");

        for row in 0..NB_SQUARE_V {
            squares.push(Vec::new());
            for column in 0..NB_SQUARE_H {
                println!("DDDDDDDD");
                let id = NB_SQUARE_H * row + column;
                println!("{} sur {}", id, NB_SQUARE);
                println!("EEEEEEEEE");
                //squares[row][column] = Square::new(row, column, id);
                squares[row].push(Square::new(row, column, id));
                println!("FFFFFFFFF");
                usable_indices.push(id);
                println!("GGGGGGGGG");
            }
        }

        println!("finnnn");

        Self { squares, usable_indices }
    }

    pub fn draw_maze(&self) {
        println!("1");
        for row_squares in &self.squares {
            println!("2");
            for square in row_squares {
                println!("Square a dessiner où row={} et column={} et id={}", square.row_index, square.column_index, square.id);
                square.draw();
                println!("finito");
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

    pub fn define_if_valide<'a>(&self, square : &'a Square) -> (Way, usize) {

        println!("Square : {:?}", square);

        let mut bottom_acces = !square.has_bottom_wall || square.row_index + 1 != NB_SQUARE_V;
        let mut right_acces = !square.has_right_wall || square.column_index + 1 != NB_SQUARE_H;

        println!("First right : {} and bottom : {}", right_acces, bottom_acces);

        if !bottom_acces & !right_acces {
            return (Way::Nothing, 0);
        }

        let mut bottom_neighbor_id = 0;
        let mut right_neighbor_id = 0;

        if bottom_acces {
            let bottom_neighbor = &self.squares[square.row_index+1][square.column_index];
            bottom_acces = square.id != bottom_neighbor.id;
            bottom_neighbor_id = bottom_neighbor.id;
        }

        if right_acces {
            let right_neighbor = &self.squares[square.row_index][square.column_index+1];
            right_acces = square.id != right_neighbor.id;
            right_neighbor_id = right_neighbor.id;
        }

        println!("Second right : {} and bottom : {}", right_acces, bottom_acces);

        match (bottom_acces, right_acces) {
            (true, true) => if rand::random_bool(0.5) { (Way::Bottom, bottom_neighbor_id) } else { (Way::Right, right_neighbor_id) },
            (true, false) => (Way::Bottom, bottom_neighbor_id),
            (false, true) => (Way::Right, right_neighbor_id),
            (false, false) => (Way::Nothing, 0),
        }

    }

    pub fn remplace_old_by_new_id(&mut self, replacing_id : usize, replaced_id : usize) {
        for row_squares in &mut self.squares {
            for square in row_squares {
                if (square.id == replaced_id) {
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