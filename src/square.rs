use ::rand::random_bool;
use macroquad::prelude::*;
use crate::params::*;

#[derive(Clone, Copy)]
pub struct Square {
    x : f32,
    y : f32,

    pub index : usize,
    pub current_id : usize,

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
        println!("{}) bottom = {} et rigth = {}", id, on_bottom_border, on_right_border);
        Square { x, y, index : id, current_id : id, has_bottom_wall : true, on_bottom_border, has_right_wall : true, on_right_border}
    }

    pub fn draw(&self) {
        let w = if self.has_right_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };
        let h = if self.has_bottom_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };

        draw_rectangle(self.x, self.y, w, h, BLUE);
    }

    pub fn draw_debug(&self) {
        self.draw();
        draw_text(&self.current_id.to_string(), self.x + SIZE_SQUARE/3.0, self.y + SIZE_SQUARE/3.0, SIZE_SQUARE/3.0, RED);
    }

    pub fn return_walls_to_destroy(&self, bottom_and_right_id : (Option<usize>, Option<usize>)) -> Way {

        // let usable_on_the_right = self.has_right_wall & !self.on_right_border;
        // let usable_at_the_bottom = self.has_bottom_wall & !self.on_bottom_border;

        // let usable_on_the_right = bottom_and_right_id.0 != self.current_id;
        // let usable_at_the_bottom = bottom_and_right_id.1 != self.current_id;

        let usable_on_the_right = match bottom_and_right_id.0 {
            Some(a) => a != self.current_id,
            None => false //unusable
        };

        let usable_at_the_bottom = match bottom_and_right_id.1 {
            Some(a) => a != self.current_id,
            None => false //unusable
        };

        println!("Self.id : {} ------- Bottom : {} ------- Right : {}",self.current_id, usable_at_the_bottom, usable_on_the_right);

        match (usable_at_the_bottom, usable_on_the_right) {
            (true, true) => if random_bool(0.5) {Way::Right} else {Way::Bottom},
            (true, false) => Way::Bottom,
            (false, true) => Way::Right,
            (false, false) => Way::Nothing,
        }
    }

}