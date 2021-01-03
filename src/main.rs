#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ggez;
use ggez::event;
use ggez::GameResult;

mod constants;
mod game;
mod drawing;
// D:\Code\Ultimate-Tic-Tac-Toe\multiplayer\TicTacToeStructs.rs
use constants::{NAME, AUTHOR, WINDOW_SIZE};
use crate::game::{Board, Player};

#[path = "../multiplayer/TicTacToeStructs.rs"]
mod TicTacToeStructs;
use crate::TicTacToeStructs::TicTacToeStructs::Message;
use crate::TicTacToeStructs::TicTacToeStructs::ClientUser;
use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;        
use crossbeam_channel::unbounded;

// const LOCAL: &str = "127.0.0.1:6000";
const LOCAL: &str = "46.252.181.151:16797";

const MSG_SIZE: usize = 128;

fn main() -> GameResult {

    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx):(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>) = unbounded();

    let (tx2, rx2):(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>) = unbounded();

    let (tx4, rx4):(crossbeam_channel::Sender<String>, crossbeam_channel::Receiver<String>) = unbounded();

    let (tx6, rx6):(crossbeam_channel::Sender<Message>, crossbeam_channel::Receiver<Message>) = unbounded();
    
    let (tx7, rx7):(crossbeam_channel::Sender<String>, crossbeam_channel::Receiver<String>) = unbounded();
    
    // let mut user = ClientUser::new();
    let mut enteredRoom:bool = false;
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                println!("got!");
                let msg = buff.into_iter().collect::<Vec<_>>();
                let msg: Message = bincode::deserialize(&msg).unwrap();
                println!("message recv {:?}", msg);
                tx2.send(msg.clone()).expect("failed to send msg to rx");
            },
            Err(ref err) if err.kind() == ErrorKind::WouldBlock =>(),
            Err(_) => {
                println!("connection with server was severed");
                break;
            }
        }
        match rx.try_recv() {
            Ok(msg) => {
                let mut buff = bincode::serialize(&msg).unwrap();
                buff.resize(MSG_SIZE, 0);
                println!("buffer {:?}", buff);
                client.write_all(&buff).expect("writing to socket failed");
                println!("message sent {:?}", msg);
            },
            Err(crossbeam_channel::TryRecvError::Empty) => (),
            Err(crossbeam_channel::TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    {
        thread::spawn(move || loop {
            let mut buff = String::new();
            io::stdin().read_line(&mut buff).expect("reading from stdin failed");
            let msg = buff.trim().to_string();
            tx4.send(msg.clone()).expect("Errored!" );
        });
        // thread::sleep(Duration::from_millis(100));
    }
    
    
    println!("Enter Room num:");
    thread::spawn(move || loop {
        if let Ok(msg) = rx2.try_recv() {
            println!("Here3!");
            
            if msg.getHeader()== "enteredRoom" {
                println!("Here2!");
                enteredRoom = true;
                tx7.send("Joined room".to_string()).expect("failed...");
            }else if msg.getHeader()== "PlayerTurn" {
                tx7.send("PlayerTurn".to_string()).expect("failed...");
            }else if msg.getHeader()== "EnemyMove" {
                tx7.send("EnemyMove".to_string() + &msg.getData()).expect("failed...");
            }else{
                println!("Enter Room num:");
            }
        }if let Ok(msg) = rx4.try_recv() {
            println!("Here4!");
            // if msg.getHeader()== "enteredRoom" {
            //     println!("Here2!");
            //     enteredRoom = true;
            // }
            // tx7.send("Sending to boardObject".to_string()).expect("failed...");
            if enteredRoom{
                //do something
            }
            else if enteredRoom || msg == ":quit" || tx.send(Message::new("JoinRoom".to_string(),msg)).is_err() {break}  
            
        }
        if let Ok(msg) = rx6.try_recv() {
            println!("Reciving from boardObject {:?}", msg);//Fprward to server
            tx.send(msg).expect("error");
            // if enteredRoom || msg == ":quit" || tx.send(Message::new("JoinRoom".to_string(),msg)).is_err() {break}  
        }
        // if let Ok(msg) = rx6.try_recv() {
        //     println!("Here6! {}",msg);
        // }
        thread::sleep(Duration::from_millis(100));
    });
    // Make a Context and an event loop
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new(NAME, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(NAME))
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1))
        .build()?;

    // create the game state with the human player going first with X
    let state = &mut Board::new(Player::X,tx6,rx7);
    // state.spawn_tx_thread();
    // launch the game by start running the event loop
    // uses the context and event loop we created above and the game state we just created
    event::run(ctx, event_loop, state)
}
