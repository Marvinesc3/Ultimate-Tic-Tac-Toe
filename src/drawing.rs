use ggez::{
    graphics::{self, MeshBuilder, Text},
    nalgebra::Point2,
    Context,
};

use crate::constants::{BOARD_POS, BOARD_SIDE, WINDOW_SIZE, SQUARE_SIZE};
use crate::game::Player;


pub fn draw_board(mb: &mut MeshBuilder) {

    for i in 0..BOARD_SIDE + 1 {
        let _ = mb.line(
            &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * i as f32, BOARD_POS.1),
                Point2::new(BOARD_POS.0 + SQUARE_SIZE * i as f32, BOARD_POS.1 + 
                    SQUARE_SIZE * BOARD_SIDE as f32,),
            ], 
            4.0, 
            graphics::BLACK,
        );
    }

    for i in 0..BOARD_SIDE + 1 {
        let _ = mb.line(
            &[  Point2::new(BOARD_POS.0, BOARD_POS.1 + SQUARE_SIZE * i as f32),
                Point2::new(BOARD_POS.0 + SQUARE_SIZE * BOARD_SIDE as f32,
                    BOARD_POS.1 + SQUARE_SIZE * i as f32,),
            ], 
            4.0, 
            graphics::BLACK,
        );
    }
}


pub fn draw_selected_cell(mb: &mut MeshBuilder, index_x: usize, index_y: usize) {

    let red_color = graphics::Color::from_rgb_u32(0x00FF0000);
    let offset = 5.0;
    let width = 5.0;
    
    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * index_x as f32 + offset,
                BOARD_POS.1 + SQUARE_SIZE * index_y as f32 + offset - 1.0,),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * (index_x + 1) as f32 - offset,
                BOARD_POS.1 + SQUARE_SIZE * index_y as f32 + offset - 1.0,),
        ], 
        width, 
        red_color,
    );

    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * index_x as f32 + offset,
                BOARD_POS.1 + SQUARE_SIZE * (index_y + 1) as f32 - offset,),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * (index_x + 1) as f32 - offset,
                BOARD_POS.1 + SQUARE_SIZE * (index_y + 1) as f32 - offset,),
        ], 
        width, 
        red_color,
    );

    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * index_x as f32 + offset,
                BOARD_POS.1 + SQUARE_SIZE * index_y as f32 + offset,),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * index_x as f32 + offset,
                BOARD_POS.1 + SQUARE_SIZE * (index_y + 1) as f32 - offset,),
        ], 
        width, 
        red_color,
    );

    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * (index_x + 1) as f32 - offset + 1.0,
                BOARD_POS.1 + SQUARE_SIZE * index_y as f32 + offset,),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * (index_x + 1) as f32 - offset + 1.0,
                BOARD_POS.1 + SQUARE_SIZE * (index_y + 1) as f32 - offset,),
        ], 
        width, 
        red_color,
    );
}


pub fn draw_red_line(mb: &mut MeshBuilder, index_first: usize, index_second: usize) {
    
    let (point1_y, point1_x) = (index_first / 3, index_first % 3);
    let (point2_y, point2_x) = (index_second / 3, index_second % 3);
    let red_color = graphics::Color::from_rgb_u32(0x00FF0000);
    
    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * point1_x as f32 + SQUARE_SIZE / 2.0,
                BOARD_POS.1 + SQUARE_SIZE * point1_y as f32 + SQUARE_SIZE / 2.0,),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * point2_x as f32 + SQUARE_SIZE / 2.0,
                BOARD_POS.1 + SQUARE_SIZE * point2_y as f32 + SQUARE_SIZE / 2.0,),
        ], 
        10.0, 
        red_color,
    );
}


pub fn draw_player_o(mb: &mut MeshBuilder, pos_x: usize, pos_y: usize) {

    mb.circle(  
        graphics::DrawMode::stroke(4.0),
        Point2::new(BOARD_POS.0 + (pos_x as f32 + 0.5) * SQUARE_SIZE,
            BOARD_POS.1 + (pos_y as f32 + 0.5) * SQUARE_SIZE,),
        SQUARE_SIZE as f32 / 4.0,
        0.00001,
        graphics::Color::from_rgb_u32(0xFFFF00),
        //graphics::BLACK,
    );
}


pub fn draw_player_x(mb: &mut MeshBuilder, pos_x: usize, pos_y: usize) {
    
    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.25),
                BOARD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.25),),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.75),
                BOARD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.75),),
        ],
        4.0,
        graphics::WHITE,
    );

    let _ = mb.line(
        &[  Point2::new(BOARD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.75),
                BOARD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.25),),
            Point2::new(BOARD_POS.0 + SQUARE_SIZE * (pos_x as f32 + 0.25),
                BOARD_POS.1 + SQUARE_SIZE * (pos_y as f32 + 0.75),),
        ],
        4.0,
        graphics::WHITE,
    );
}


pub fn draw_player(mb: &mut MeshBuilder, player: Player, pos_x: usize, pos_y: usize) {

    match player {
        Player::O => draw_player_o(mb, pos_x, pos_y),
        Player::X => draw_player_x(mb, pos_x, pos_y),
    } 
}


pub fn draw_text(ctx: &mut Context, text: &str) {

    let display_text = Text::new(format!("Game: {}\nPress 'R' to restart", text));

    let _ = graphics::draw( ctx, &display_text, (Point2::new(0.0, WINDOW_SIZE.1 * 0.9), 
        graphics::WHITE),
    );
}
