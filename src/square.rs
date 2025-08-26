use macroquad::{color, prelude::*};
use crate::params::*;


#[derive(Clone, Copy, Debug)]
pub struct Square {
    x : f32,
    y : f32,

    pub id : usize,

    pub has_bottom_wall : bool,
    pub on_bottom_border : bool,
    pub has_right_wall : bool,
    pub on_right_border : bool,
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
    pub fn new (x : f32, y : f32, id : usize, on_bottom_border : bool, on_right_border : bool) -> Self {
        Square { x, y, id, has_bottom_wall : true, on_bottom_border, has_right_wall : true, on_right_border}
    }

    pub fn draw(&self) {
        let w = if self.has_right_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };
        let h = if self.has_bottom_wall { SIZE_SQUARE-BORDER_SQUARE } else { SIZE_SQUARE };

        draw_rectangle(self.x, self.y, w, h, color::SKYBLUE);
    }

    pub fn draw_debug(&self) {
        self.draw();
        draw_text(&self.id.to_string(), self.x + SIZE_SQUARE/3.0, self.y + SIZE_SQUARE/3.0, SIZE_SQUARE/3.0, RED);
    }

    pub fn break_wall(&mut self, way : Way) {
        println!("mur a detruire = {:?}", way);
        println!("{:?}", self);
        match way {
            Way::Right => self.has_right_wall = false,
            Way::Bottom => self.has_bottom_wall = false,
            _ => return,
        }
        println!("{:?}", self);
    }

    ///Return if there is acces without considering neighbor's id
    pub fn return_bottom_and_right_acces(&self) -> (bool, bool) {
        let bottom_acces = self.has_bottom_wall & !self.on_bottom_border;
        let right_acces = self.has_right_wall & !self.on_right_border;

        (bottom_acces, right_acces)
    }

    pub fn filter_by_wall(&self, ways : Vec<Way>, select_walls : bool) -> Vec<Way> {
        let mut output = Vec::new();

        for way in ways {
            match way {
                Way::Right => if self.has_right_wall == select_walls { output.push(Way::Right); },
                Way::Bottom => if self.has_bottom_wall == select_walls { output.push(Way::Bottom); },
                _ => continue
            }
        }
        output
    }

    pub fn filter_by_border(&self, ways : Vec<Way>) -> Vec<Way> {
        let mut output = Vec::new();

        for way in ways {
            match way {
                Way::Right => if !self.on_right_border { output.push(Way::Right); },
                Way::Bottom => if !self.on_bottom_border { output.push(Way::Bottom); },
                _ => continue
            }
        }
        output
    }

}

#[cfg(test)]
mod tests {
    use crate::square;

    use super::*;

    #[test]
    fn check_filter_by_wall() {
        let mut square = Square{ x: 0.0, y: 0.0, id: 0, has_bottom_wall: true, on_bottom_border: false, has_right_wall: true, on_right_border: false };

        assert_eq!(square.filter_by_wall(vec![Way::Bottom, Way::Right], false), Vec::new());
        assert_eq!(square.filter_by_wall(vec![Way::Bottom, Way::Right, Way::Nothing], true), vec![Way::Bottom, Way::Right]);

        square.has_right_wall = false;

        assert_eq!(square.filter_by_wall(vec![Way::Bottom, Way::Right], false), vec![Way::Right]);
        assert_eq!(square.filter_by_wall(vec![Way::Bottom, Way::Right, Way::Nothing], true), vec![Way::Bottom]);
    }

    #[test]
    fn check_filter_by_border() {
        let mut square = Square{ x: 0.0, y: 0.0, id: 0, has_bottom_wall: false, on_bottom_border: true, has_right_wall: false, on_right_border: true };

        assert_eq!(square.filter_by_border(vec![Way::Bottom, Way::Right]), Vec::new());

        square.on_bottom_border = false;

        assert_eq!(square.filter_by_border(vec![Way::Bottom, Way::Right]), vec![Way::Bottom]);
    }
}