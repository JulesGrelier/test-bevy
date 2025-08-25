use macroquad::prelude::*;
use crate::params::*;

#[derive(Clone, Copy)]
pub struct Square {
    x : f32,
    y : f32,

    pub id : usize,

    pub has_bottom_wall : bool,
    pub on_bottom_border : bool,
    pub has_right_wall : bool,
    pub on_right_border : bool,
}

#[derive(PartialEq, Debug)]
pub enum Way {
    Right,
    Bottom,
    Nothing
}

impl Square {
    pub fn new (x : f32, y : f32, id : usize, on_bottom_border : bool, on_right_border : bool) -> Self {
        Square { x, y, id, has_bottom_wall : true, on_bottom_border, has_right_wall : true, on_right_border}
    }

    pub fn draw(&self) {
        let w = if self.has_right_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };
        let h = if self.has_bottom_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };

        draw_rectangle(self.x, self.y, w, h, RED);
    }

    pub fn draw_debug(&self) {
        self.draw();
        draw_text(&self.id.to_string(), self.x + SIZE_SQUARE/3.0, self.y + SIZE_SQUARE/3.0, SIZE_SQUARE/3.0, RED);
    }

    pub fn break_wall(&mut self, way : Way) {
        match way {
            Way::Right => self.has_right_wall = false,
            Way::Bottom => self.has_bottom_wall = false,
            Way::Nothing => return,
        }
    }

    ///Return if there is acces without considering neighbor's id
    pub fn return_bottom_and_right_acces(&self) -> (bool, bool) {
        let bottom_acces = self.has_bottom_wall & !self.on_bottom_border;
        let right_acces = self.has_right_wall & !self.on_right_border;

        (bottom_acces, right_acces)
    }

}