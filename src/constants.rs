pub static NAME: &str = "TicTacToe GUI";
pub static AUTHOR: &str = "BCA Capstone";

pub static WINDOW_SIZE: (f32, f32) = (960.0, 640.0);

pub static SQUARE_SIZE: f32 = 150.0;

pub static BOARD_SIDE: usize = 3;

pub static BOARD_POS: (f32, f32) = (
    WINDOW_SIZE.0 / 2.0 - SQUARE_SIZE * (BOARD_SIDE as f32 / 2.0),
    WINDOW_SIZE.1 / 2.0 - SQUARE_SIZE * (BOARD_SIDE as f32 / 2.0),
);
