use macroquad::{color, prelude::*};
use crate::params::*;


#[derive(Clone, Copy, Debug)]
pub struct Square {

    pub row_index : usize,
    y : f32,

    pub column_index : usize,
    x : f32,

    pub id : usize,

    pub has_bottom_wall : bool,
    pub has_right_wall : bool,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Way {
    Top,
    Left,
    Right,
    Bottom,
    Nothing
}

impl Square {

    pub fn new(row_index : usize, column_index : usize, id : usize) -> Self {
        Self {
            row_index,
            y: row_index as f32 * SIZE_SQUARE,

            column_index,
            x: column_index as f32 * SIZE_SQUARE,

            id,
            has_bottom_wall: true,
            has_right_wall: true
        }
    }

    pub fn draw(&self) {
        let w = if self.has_right_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };
        let h = if self.has_bottom_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };

        draw_rectangle(self.x, self.y, w, h, color::SKYBLUE);
    }

    pub fn debug(&self) {
        self.draw();
        draw_text(&self.id.to_string(), self.x + SIZE_SQUARE/3.0, self.y + SIZE_SQUARE/3.0, SIZE_SQUARE/3.0, RED);
    }

    pub fn break_wall(&mut self, way : Way) {
        match way {
            Way::Right => self.has_right_wall = false,
            Way::Bottom => self.has_bottom_wall = false,
            _ => return,
        }
    }
}