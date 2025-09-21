
// use rand::Rng;

// use crate::{params::*, square};
// use crate::square::{Way, Square};

// pub struct Game {
//     squares : Vec<Square>,
//     usable_indices : Vec<usize>,
// }

// pub enum Situation {
//     Success,
//     EmptyUsableIndices,
//     UselessIndex(usize)
// }

// impl Game {
//     pub fn new() -> Self {
//         let mut squares = Vec::with_capacity(NB_SQUARE as usize);
//         let mut usable_indices = Vec::with_capacity(NB_SQUARE as usize);

//         for id in 0..NB_SQUARE as usize {
//             let f32_id = id as f32;

//             let x = (SIZE_SQUARE*f32_id)%WINDOW_WIDTH ;
//             let y = SIZE_SQUARE*(f32_id/NB_SQUARE_H).floor();

//             let on_bottom_border = (NB_SQUARE - f32_id) <= (NB_SQUARE_H);
//             let on_right_border = (f32_id+1.0) % NB_SQUARE_H == 0.0;

//             squares.push(Square::new(x, y, id, on_bottom_border, on_right_border));
//             usable_indices.push(id);
//         }

//         Self { squares, usable_indices }
//     }

//     pub fn draw_labyrinth(&self) {
//         for square in &self.squares {
//             square.draw();
//         }
//     }

//     pub fn debug_labyrinth(&self) {
//         for square in &self.squares {
//             square.draw_debug();
//         }
//     }

//     pub fn make_one_cycle(&mut self) -> Situation {

//         if self.usable_indices.len() == 0 {
//             return Situation::EmptyUsableIndices;
//         }

//         let usable_index = self.usable_indices[rand::rng().random_range(0..self.usable_indices.len())];
//         let square = &self.squares[usable_index];

//         let mut ways = vec![Way::Right, Way::Bottom];
//         ways = square.filter_by_wall(ways, true);
//         ways = square.filter_by_border(ways);

//         if ways.is_empty() {
//             return Situation::UselessIndex(usable_index);
//         }

//         let (neighbors, ways) = self.return_neighbors(usable_index, ways);
//         let valide_neighbors_index = return_correct_indices(&neighbors, square.id);

//         if valide_neighbors_index.is_empty() {
//             return Situation::UselessIndex(usable_index);
//         }

//         let random_nb = rand::rng().random_range(0..valide_neighbors_index.len());
//         let random_index = valide_neighbors_index[random_nb];

//         self.remplace_old_by_new_id(square.id, neighbors[random_index].id);
//         self.squares[usable_index].break_wall(ways[random_index]);

//         return Situation::Success;

//     }

//     fn return_neighbors(&self, index : usize, ways : Vec<Way>) -> (Vec<&Square>, Vec<Way>) {

//         let nb_sq = NB_SQUARE as usize;
//         let nb_sq_h = NB_SQUARE_H as usize;

//         let mut output_squares = Vec::new();
//         let mut output_ways = Vec::new();

//         for way in ways {
//             match way {
//                 Way::Top => if index >= nb_sq_h {
//                     output_squares.push(&self.squares[index-nb_sq_h]);

//                 },
//                 Way::Bottom => if index < (nb_sq-nb_sq_h) {
//                     output_squares.push(&self.squares[index+nb_sq_h]);
//                 },
//                 Way::Left => if index % nb_sq_h > 0 {
//                     output_squares.push(&self.squares[index-1]);
//                 },
//                 Way::Right => if index % nb_sq_h < nb_sq_h {
//                     output_squares.push(&self.squares[index+1]);
//                 },
//                 Way::Nothing => continue,
//             }
//             output_ways.push(way);
//         }

//         (output_squares, output_ways)
//     }

//     pub fn remove_square_from_usable_indices(&mut self, index : usize) {
//         let index_to_remove = self.usable_indices.iter().position(|&x| x == index).unwrap();
//         self.usable_indices.remove(index_to_remove);
//     }

//     pub fn remplace_old_by_new_id(&mut self, replacing_id : usize, replaced_id : usize) {
//         for square in &mut self.squares {
//             if square.id == replaced_id {
//                 square.id = replacing_id;
//             }
//         }
//     }
// }

//     ///Correct == diffrent id 
//     fn return_correct_indices(neighbors : &Vec<&Square>, id : usize) -> Vec<usize> {
//         let mut output = Vec::new();

//         for (index, neighbor) in neighbors.iter().enumerate() {
//             if (neighbor.id) != (id) { output.push(index); }
//         }
//         output
//     } 