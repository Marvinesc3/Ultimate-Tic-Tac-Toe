//! Strapped the GUI on top of a previous terminal (termion) based version
//! Didn't reactor all the game logic code, currently everything is just bolted on top of the existing Board structure
//! instead of implementing a MainState struct for the game state
//! separate from the Board struct. Sorry for the ugly code.
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use std::sync::mpsc::Sender;
use crate::drawing::*;
use crossbeam_channel::*;
use crossbeam_channel::unbounded;
use std::sync::atomic::{AtomicBool, Ordering};
use ggez::{
    event::{self, KeyCode, KeyMods, MouseButton},
    graphics::{self, DrawParam, MeshBuilder},
    Context, GameResult,
};
#[path = "../multiplayer/TicTacToeStructs.rs"]
mod TicTacToeStructs;
use crate::TicTacToeStructs::TicTacToeStructs::Message;

use crate::constants::{BOARD_POS, BOARD_SIDE, SQUARE_SIZE};


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Player {
    X,
    O,
}


impl Player {
    pub fn opponent(&self) -> Player {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}


// UI related enums
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SelectedCell {
    NotSelected,
    Selected { x: usize, y: usize },
}


#[derive(PartialEq)]
pub enum PointingWhereType {
    InsideTheBoard,
    OutsideTheBoard,
}


pub enum Directions {
    Left,
    Up,
    Right,
    Down,
}


#[derive(Debug, PartialEq)]
pub enum GameState {
    GameWon { player: Player, cells: Vec<usize> },
    Tie,
    InProgress,
}


// Main game state
// #[derive(Copy, Clone)]

#[derive(Debug)]
pub struct Board {
    pub fields: [[Option<Player>; 3]; 3],
    pub next_player: Player,
    pub selected_cell: SelectedCell,
    pub players_turn: bool,
    pub joined_room: bool,
    pub tx: crossbeam_channel::Sender<Message>,
    pub rx: crossbeam_channel::Receiver<String>,
    pub tx2: crossbeam_channel::Sender<String>,
    pub rx2: crossbeam_channel::Receiver<String>,
    
    // pub tx:Sender<std::string::String>,
    // pub rx: std::sync::mpsc::Receiver<std::string::String>,
}   


impl Board {

    pub fn new(first_player: Player, tx: crossbeam_channel::Sender<Message>, rx:crossbeam_channel::Receiver<String>) -> Board {
        let (tx2, rx2):(crossbeam_channel::Sender<String>, crossbeam_channel::Receiver<String>) = unbounded();
        let mut board = Board {
            fields: [
                [None, None, None],
                [None, None, None],
                [None, None, None]
            ],
            next_player: first_player,
            selected_cell: SelectedCell::NotSelected,
            players_turn: false,
            joined_room: false,
            tx: tx.clone(),
            rx,
            tx2,
            rx2,
        };
        // let board_ref =  &board;
        // thread::spawn(move || loop {
        //     if let Ok(msg) = rx.try_recv() {
        //         // board.joined_room =false;
        //         board_ref.spawn_rx_thread();
        //     }
        //     thread::sleep(Duration::from_millis(100));
        // });
        board.spawn_rx_thread();
        board
    }

    pub fn spawn_rx_thread(&mut self){
        let rx = self.rx.clone();
        let tx2 =self.tx2.clone();
        thread::spawn(move || loop {
            if let Ok(msg) = rx.try_recv() {
                println!("IN THE RX {}", msg);
                // joined_room = true;
                
                tx2.send(msg).expect("Error");
            }
            thread::sleep(Duration::from_millis(100));
        });
    }

    // pub fn spawn_tx_thread(&mut self){
    //     loop{
    //         if let Ok(msg) = self.rx2.try_recv() {
    //             println!("Object rx2! {}", msg);
    //             self.joined_room = true;
    //         }
    //         thread::sleep(Duration::from_millis(100));
    //     }
    // }

    pub fn next_player(&self) -> Player {

        self.next_player
    }


    // available cells (with None)
    fn available_cells(&self) -> bool {
    	
    	self.fields.iter().all(|row| {
                row.iter().all(|cell| cell.is_some())
        })
    }


    pub fn is_ended(&self) -> bool {
        
        let game_state = self.get_winner(); 
        match &game_state {

            GameState::GameWon { player: _, cells: _ } => true,

            _ => self.available_cells(),
        }
    }


    pub fn get_winner(&self) -> GameState {
        
        macro_rules! has {
            ($player:expr, $x:expr, $y:expr) => {
                self.fields[$x][$y] == Some(*$player)
            };
        }

        // check for both players in turn
        for player in &[Player::X, Player::O] {

            // Three in a row: horizontally
            for row in 0..=2 {
                if has!(player, row, 0) && has!(player, row, 1) && has!(player, row, 2) {
                    return GameState::GameWon {
                        player: *player,
                        cells: vec![row * 3, row * 3 + 1, row * 3 + 2],
                    };
                }
            }

            // Three in a row: vertically
            for col in 0..=2 {
                if has!(player, 0, col) && has!(player, 1, col) && has!(player, 2, col) {
                    return GameState::GameWon {
                        player: *player,
                        cells: vec![col, col + 3, col + 6],
                    };
                }
            }

            // Three in a row: diagonally (top-left to bottom-right)
            if has!(player, 0, 0) && has!(player, 1, 1) && has!(player, 2, 2) {
                return GameState::GameWon {
                    player: *player,
                    cells: vec![0, 4, 8],
                };
            }

            // Three in a row: diagonally (top-right to bottom-left)
            if has!(player, 0, 2) && has!(player, 1, 1) && has!(player, 2, 0) {
                return GameState::GameWon {
                    player: *player,
                    cells: vec![2, 4, 6],
                };
            }
        }

        // if there are no empty cells return a Tie if not return InProgress
        if self.available_cells() {
            return GameState::Tie;
        } else {
            return GameState::InProgress;
        }
    }


    pub fn is_legal_action(&self, action: (i32, i32)) -> bool {

        if action.0 < 0 || action.0 > 2 || action.1 < 0 || action.1 > 2 {
            return false;
        }
        self.fields[action.0 as usize][action.1 as usize].is_none()
    }


    pub fn perform_action(&mut self, action: (i32, i32)) {

        // Perform...
        self.fields[action.0 as usize][action.1 as usize] = Some(self.next_player);

        // Next player's turn
        self.next_player = self.next_player.opponent();
    }


    pub fn get_actions(&self) -> Vec<(i32, i32)> {

        if self.is_ended() {
            return Vec::new();
        }

        let mut actions = Vec::with_capacity(9);

        // Calculate possible moves
        for row in 0..3 {
            for col in 0..3 {
                if self.is_legal_action((row, col)) {
                    actions.push((row, col));
                }
            }
        }

        actions
    }


    // UI Related methods

    pub fn get_pointing_where_type(&self, x: f32, y: f32) -> PointingWhereType {
        
        if BOARD_POS.0 < x && x < BOARD_POS.0 + SQUARE_SIZE * BOARD_SIDE as f32
            && BOARD_POS.1 < y && y < BOARD_POS.1 + SQUARE_SIZE * BOARD_SIDE as f32 {

            return PointingWhereType::InsideTheBoard;
        }

        PointingWhereType::OutsideTheBoard
    }


    pub fn get_cell(&self, x: f32, y: f32) -> (usize, usize) {
        let cell_x = (x - BOARD_POS.0) / SQUARE_SIZE;
        let cell_y = (y - BOARD_POS.1) / SQUARE_SIZE;

        (cell_x as usize, cell_y as usize)
    }


    // As soon as we have valid user input perform user action and then AI action
    fn perform_both_turns(&mut self, user_action: (i32, i32)) {
        // self.next_player();
        if  self.players_turn == true{
            let  sendMsg:String ="".to_string() + &user_action.0.to_string() + ","+ &user_action.1.to_string();
            let message: Message = Message::new("Move".to_string(),sendMsg);
            // sendMsg.push_str(user_action.0.to_string());
            // sendMsg = sendMsg + &user_action.0.to_string();
            self.tx.send(message).expect("failed to send msg to rx");
        }
        
        self.perform_action(user_action);
        // As Human is 1st player check if game has not ended before AI's turn
        if !self.is_ended() {
            // let ai_action = find_best_move(*self, self.next_player);
            // self.perform_action(ai_action);
        }
    }


    fn select_cell(&mut self, x: f32, y: f32) {
        let (index_x, index_y) = self.get_cell(x, y);
        self.selected_cell = SelectedCell::Selected {
            x: index_x,
            y: index_y,
        };
    }


    fn move_selected_cell(&mut self, direction: Directions) {
        
        if self.is_ended() {
            return
        }

        if self.selected_cell == SelectedCell::NotSelected {
            self.selected_cell = SelectedCell::Selected { x: 0, y: 0 };
            return;
        }

        match direction {
            
            Directions::Down => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if y < BOARD_SIDE - 1 {
                        self.selected_cell = SelectedCell::Selected { x: x, y: y + 1 };
                    }
                }
            } // end Down

            Directions::Up => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if y > 0 {
                        self.selected_cell = SelectedCell::Selected { x: x, y: y - 1 };
                    }
                }
            } // end Up

            Directions::Right => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if x < BOARD_SIDE - 1 {
                        self.selected_cell = SelectedCell::Selected { x: x + 1, y: y };
                    }
                }
            } // end Right

            Directions::Left => {
                if let SelectedCell::Selected { x, y } = self.selected_cell {
                    if x > 0 {
                        self.selected_cell = SelectedCell::Selected { x: x - 1, y: y };
                    }
                }
            } // end Left

        } // end match
    }


    fn check_valid_action_on_selected_cell(&mut self, _player: Player) -> bool {
        match self.selected_cell {
            SelectedCell::NotSelected => return false,
            SelectedCell::Selected { x, y } => {
                let action = (y as i32, x as i32); // row and column
                let possible_moves = self.get_actions();
                if possible_moves.contains(&action) {
                    return true
                } else {
                    return false
                }
            }
        }
    }

    fn get_selected_cell(&self) -> SelectedCell {
        self.selected_cell.clone()
    }

}


// UI Board (game state) Event Handler

impl event::EventHandler for Board {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if let Ok(msg) = self.rx2.try_recv() {
            println!("Object rx2! {}", msg);
            if msg.contains("Joined room"){
                self.joined_room = true;
            }else if msg.contains("EnemyMove"){
                let vec = (msg["EnemyMove".len()..].split(",")).collect::<Vec<&str>>();
                let x = i32::from(String::from(vec[0]).parse::<i32>().unwrap()).clone();
                let y = i32::from(String::from(vec[1]).parse::<i32>().unwrap()).clone();
                
                // let user_action = (vec[0].clone().parse::<i32>().unwrap(),vec[1].clone().parse::<i32>().unwrap());
                self.perform_both_turns((x,y));
                self.players_turn = true;
            }else if msg.contains("PlayerTurn"){
                println!("PLAYER TURN IN OBJECT");
                self.players_turn = true;
            }
            
        }
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        
        if !self.is_ended() {
            if button == MouseButton::Left {
                let pointing_where = self.get_pointing_where_type(x, y);
                if pointing_where == PointingWhereType::InsideTheBoard {
                    let cell = self.get_cell(x, y);
                    let user_action = (cell.1 as i32, cell.0 as i32); // row, column
                    if self.is_legal_action(user_action) && self.joined_room && self.players_turn{
                        self.perform_both_turns(user_action);
                        self.players_turn = false;
                    }
                }
            }
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        
        if !self.is_ended() {
            let pointing_where = self.get_pointing_where_type(x, y);
            if pointing_where == PointingWhereType::InsideTheBoard {
                self.select_cell(x, y);
            }
        }
    }

    fn key_down_event(&mut self , _ctx: &mut Context, keycode: KeyCode, 
        _keymod: KeyMods, _repeat: bool,) {

        match keycode {
            // KeyCode::R => *self = Board::new(Player::X,tx),
            KeyCode::Left => self.move_selected_cell(Directions::Left),
            KeyCode::Right => self.move_selected_cell(Directions::Right),
            KeyCode::Up => self.move_selected_cell(Directions::Up),
            KeyCode::Down => self.move_selected_cell(Directions::Down),
            KeyCode::Space => {
                if !self.is_ended() {
                    if self.check_valid_action_on_selected_cell(Player::X) && self.joined_room && self.players_turn {
                        if let SelectedCell::Selected { x, y } = self.get_selected_cell() {
                            let action = (y as i32, x as i32); // row and column
                            self.perform_both_turns(action);
                            self.players_turn = false;
                        }
                    }
                }
            }
            _ => (),
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // clear the window
        //graphics::clear(_ctx, graphics::Color::from_rgb_u32(0xB0B0B0));
        graphics::clear(ctx, [0.2, 0.8, 0.2, 1.0].into());

        // draw stuff using a MeshBuilder
        let mb = &mut MeshBuilder::new();

        // draw the board
        draw_board(mb);

        // draw the players        
        for (i, row) in self.fields.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                match *cell {
                    Some(Player::X) => draw_player(mb, Player::X, j, i),
                    Some(Player::O) => draw_player(mb, Player::O, j, i),
                    None => {},
                };
            }
        }

        let game_state = self.get_winner(); 
        match &game_state {
            GameState::GameWon { player: _, cells } => {
                // draw line if somebody won
                draw_red_line(mb, cells[0], cells[2]);
            },
            GameState::InProgress => {
                if let SelectedCell::Selected { x, y } = self.get_selected_cell() {
                    // draw selected cell
                    draw_selected_cell(mb, x, y);
                }
            },
            _ => (),
        }
        
        // draw the text
        let text = game_state_to_str(&game_state);
        // println!("{:?}",self);
        draw_text(ctx, &text);
        // build the mesh
        let mbb = mb.build(ctx)?;
        ggez::graphics::draw(ctx, &mbb, DrawParam::default())?;

        // flip the screen - switch buffer to screen
        graphics::present(ctx)?;

        // geez::timer signals OS it does not need all the CPU time just for this game
        // prevents the game from using 100% CPU all the time
        ggez::timer::yield_now();

        // returns an ok GameResult if no errors
        Ok(())
    }

}


// Utility funtions for the UI
    
fn game_state_to_str(game_state: &GameState) -> String {
    match game_state {
        GameState::Tie => String::from("Tie"),
        GameState::InProgress => String::from("In progress"),
        GameState::GameWon { player, .. } => match player {
            Player::O => String::from("Computer won"),
            Player::X => String::from("Player won"),
        },
    }
}


