pub const WINDOW_WIDTH : f32 = 1080.0;
pub const WINDOW_HEIGHT : f32 = 720.0;

pub const NB_SQUARE_H: usize = (WINDOW_WIDTH / SIZE_SQUARE) as usize;
pub const NB_SQUARE_V: usize = (WINDOW_HEIGHT / SIZE_SQUARE) as usize;
pub const NB_SQUARE : usize = (NB_SQUARE_H * NB_SQUARE_V) as usize;

pub const SIZE_SQUARE : f32 = 6.0;
pub const BORDER_SQUARE : f32 = 1.0;