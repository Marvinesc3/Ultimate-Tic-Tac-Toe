//! Strapped the GUI on top of a previous terminal (termion) based version
//! Didn't reactor all the game logic code, currently everything is just bolted on top of the existing Board structure
//! instead of implementing a MainState struct for the game state
//! separate from the Board struct. Sorry for the ugly code.

use crate::drawing::*;

use ggez::{
    event::{self, KeyCode, KeyMods, MouseButton},
    graphics::{self, DrawParam, MeshBuilder},
    Context, GameResult,
};

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
#[derive(PartialEq, Copy, Clone)]
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
#[derive(Copy, Clone)]
pub struct Board {
    pub fields: [[Option<Player>; 3]; 3],
    pub next_player: Player,
    pub selected_cell: SelectedCell,
}


impl Board {

    pub fn new(first_player: Player) -> Board {
        Board {
            fields: [
                [None, None, None],
                [None, None, None],
                [None, None, None]
            ],
            next_player: first_player,
            selected_cell: SelectedCell::NotSelected,
        }
    }


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
    
        debug_assert!(self.is_legal_action(action));

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
        self.next_player();
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
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        
        if !self.is_ended() {
            if button == MouseButton::Left {
                let pointing_where = self.get_pointing_where_type(x, y);
                if pointing_where == PointingWhereType::InsideTheBoard {
                    let cell = self.get_cell(x, y);
                    let user_action = (cell.1 as i32, cell.0 as i32); // row, column
                    if self.is_legal_action(user_action) {
                        self.perform_both_turns(user_action);
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
            KeyCode::R => *self = Board::new(Player::X),
            KeyCode::Left => self.move_selected_cell(Directions::Left),
            KeyCode::Right => self.move_selected_cell(Directions::Right),
            KeyCode::Up => self.move_selected_cell(Directions::Up),
            KeyCode::Down => self.move_selected_cell(Directions::Down),
            KeyCode::Space => {
                if !self.is_ended() {
                    if self.check_valid_action_on_selected_cell(Player::X) {
                        if let SelectedCell::Selected { x, y } = self.get_selected_cell() {
                            let action = (y as i32, x as i32); // row and column
                            self.perform_both_turns(action);
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


// tests

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn check_opponent() {
        let player_x = Player::X;
        let player_o = Player::O;
        assert_eq!(player_x.opponent(), player_o);
        assert_eq!(player_o.opponent(), player_x);
    }

    #[test]
    fn check_new_empty_board() {
        let board = Board::new(Player::X);
        assert_eq!(board.fields, [
            [None, None, None],
            [None, None, None],
            [None, None, None]
        ]);
    }

    #[test]
    fn check_get_actions() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     None]
        ];
        let manual = vec![(0,0), (1,1), (2,2)]; // row and column
        assert_eq!(board.get_actions(), manual);
    }


    #[test]
    fn check_is_legal_action() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,               Some(Player::X),     Some(Player::O)],
            [Some(Player::X),    None,                Some(Player::X)],
            [Some(Player::O),    Some(Player::X),     None]
        ];
        let mut action = (4, 3);
        assert!(!board.is_legal_action(action));
        action = (1, 1);
        assert!(board.is_legal_action(action));
    }

    #[test]
    fn check_perform_action() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     None]
        ];
        let action = (2, 2);
        board.perform_action(action);

        let mut board2 = Board::new(Player::X);
        board2.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     Some(Player::X)]
        ];

        assert_eq!(board.fields, board2.fields);
    }

    #[test]
    fn check_board_cloning() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   None,                Some(Player::X)],
            [Some(Player::O),   Some(Player::X),     None]
        ];

        let board2 = board.clone();
        assert_eq!(board.fields, board2.fields);
    }

    #[test]
    fn check_is_ended_winner() {
        let mut board = Board::new(Player::X);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   Some(Player::X),     Some(Player::X)],
            [Some(Player::O),   Some(Player::O),     None]
        ];
        assert!(board.is_ended()); // X Won

    }

    #[test]
    fn check_is_ended_no_moves() {
        let mut board = Board::new(Player::X);

        board.fields = [
            [Some(Player::O),   Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   Some(Player::O),     Some(Player::X)],
            [Some(Player::X),   Some(Player::O),     Some(Player::X)]
        ];
        assert!(board.is_ended()); // No more fields available with None

    }

    
    #[test]
    fn check_get_winner() {
        let mut board = Board::new(Player::O);
        board.fields = [
            [None,              Some(Player::X),     Some(Player::O)],
            [Some(Player::X),   Some(Player::X),     Some(Player::X)],
            [Some(Player::O),   Some(Player::O),     None]
        ];

        let mut manual_winner = Player::X;

        //assert_eq!(board.get_winner().unwrap(), manual_winner);
        let game_state = board.get_winner();
        match game_state {
            GameState::Tie => {},
            GameState::InProgress => {},
            GameState::GameWon { player, .. } => match player {
                Player::O => assert_eq!(Player::O, manual_winner),
                Player::X => assert_eq!(Player::X, manual_winner),
            },
        }

        board.fields = [
            [None,              None,                None],
            [None,              None,                None],
            [Some(Player::O),   Some(Player::O),     Some(Player::O)]
        ];

        manual_winner = Player::O;
        
        //assert_eq!(board.get_winner().unwrap(), manual_winner); 
        let game_state = board.get_winner();
        match game_state {
            GameState::Tie => {},
            GameState::InProgress => {},
            GameState::GameWon { player, .. } => match player {
                Player::O => assert_eq!(Player::O, manual_winner),
                Player::X => assert_eq!(Player::X, manual_winner),
            },
        }
    }
    
    
    
}
