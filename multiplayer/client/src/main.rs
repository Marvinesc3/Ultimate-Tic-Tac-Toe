use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

#[path = "../../TicTacToeStructs.rs"]
mod TicTacToeStructs;
use crate::TicTacToeStructs::TicTacToeStructs::Message;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 128;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Stream failed to connect");
    client.set_nonblocking(true).expect("failed to initiate non-blocking");

    let (tx, rx) = mpsc::channel::<Message>();

    let mut enteredRoom = false;
    thread::spawn(move || loop {
        let mut buff = vec![0; MSG_SIZE];
        match client.read_exact(&mut buff) {
            Ok(_) => {
                println!("got!");
                let msg = buff.into_iter().collect::<Vec<_>>();
                let msg: Message = bincode::deserialize(&msg).unwrap();
                println!("message recv {:?}", msg);
                if msg.getHeader()== "enteredRoom" {
                    println!("Here!");
                    enteredRoom = true;
                }
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
            Err(TryRecvError::Empty) => (),
            Err(TryRecvError::Disconnected) => break
        }

        thread::sleep(Duration::from_millis(100));
    });

    println!("Enter Room num:");
    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("reading from stdin failed");
        let msg = buff.trim().to_string();
        if !enteredRoom {
            enteredRoom = true;
            if msg == ":quit" || tx.send(Message::new("JoinRoom".to_string(),msg)).is_err() {break}  
        }
        else{
            if msg == ":quit" || tx.send(Message::new("SendMsg".to_string(),msg)).is_err() {break}  
        }
        
    }
    println!("bye bye!");

}
//{header: "SendMsg".to_string(),  msg}