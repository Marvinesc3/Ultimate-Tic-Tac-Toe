use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 128;
// const PORT: &str= ":6000";
fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

#[path = "../../TicTacToeStructs.rs"]
mod TicTacToeStructs;
use crate::TicTacToeStructs::TicTacToeStructs::Room;
use crate::TicTacToeStructs::TicTacToeStructs::User;
use crate::TicTacToeStructs::TicTacToeStructs::Message;
use crate::TicTacToeStructs::TicTacToeStructs::ServerMessage;
use crate::TicTacToeStructs::TicTacToeStructs::Rooms;

use std::net::Ipv4Addr;


fn main() {
    let PORT: &str = &*std::env::var("PORT").unwrap().to_owned();
    // let PORT: &str= "6000";
    // println!("here1?");
    let addr = Ipv4Addr::UNSPECIFIED;
    let IP = std::env::var("IP").unwrap().to_owned();
    println!("local ip address: {:?}", addr);
    println!("here?");
    let localIp = IP+":" +PORT; 
    println!("Local Ip:PORT {}", localIp);
    let server = TcpListener::bind(localIp).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");
    let mut Rooms = Rooms::new();
    Rooms.addRoom("123".to_string());
    Rooms.addRoom("2".to_string());
    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<ServerMessage>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff.into_iter().collect::<Vec<_>>();
                        let msg: Message = bincode::deserialize(&msg).unwrap();
                        tx.send(ServerMessage::new(User::new( socket.try_clone().expect("failed to clone client"), addr),msg)).expect("failed to send msg to rx");
                    }, 
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("closing connection with: {}", addr);
                        break;
                    }
                }

                sleep();
            });
        }

        if let Ok(msg) = rx.try_recv() {
            if msg.getMessage().getHeader() == "JoinRoom" {
                println!("msg:{:?}",msg);
                if Rooms.addMemberToRoom(msg.getMessage().getData(), msg.getUser()) {
                    println!("found room: ");
                    let numOfPlayers = Rooms.findRoom(msg.getMessage().getData()).unwrap().getNumberOfPlayers();
                    let currentPlayerTurn = Rooms.findRoom(msg.getMessage().getData()).unwrap().getPlayersTurn();
                    let mut buff = bincode::serialize(&Message::new("enteredRoom".to_string(),
                        numOfPlayers.to_string())).unwrap();
                    let mut temp = msg.getUser().getStream().try_clone().expect("failed to clone client");
                    buff.resize(MSG_SIZE, 0);
                    if let Err(io_error) = temp.write_all(&buff) {
                        println!("io error {}", io_error);
                    }
                    if numOfPlayers == currentPlayerTurn{
                        let mut buff = bincode::serialize(&Message::new("PlayerTurn".to_string(),
                        "".to_string())).unwrap();
                        let mut temp = msg.getUser().getStream().try_clone().expect("failed to clone client");
                        buff.resize(MSG_SIZE, 0);
                        if let Err(io_error) = temp.write_all(&buff) {
                            println!("io error {}", io_error);
                        }
                    }
                }else{
                    println!("did not find room");
                    let mut buff = bincode::serialize(&Message::new("Error".to_string(),"error finding room".to_string())).unwrap();
                    // println!("buff {:?} len:{}",buff, buff.len());
                    let mut temp = msg.getUser().getStream().try_clone().expect("failed to clone client");
                    // let mut buff = bincode::serialize(&msg.getMessage()).unwrap();
                    buff.resize(MSG_SIZE, 0);
                    if let Err(io_error) = temp.write_all(&buff) {
                        println!("io error {}", io_error);
                    }               
                }

            }else if msg.getMessage().getHeader() == "Move"{
                println!("Move!");
                Rooms.MoveWithAddr(msg.getMessage().getData(),msg.getUser().getAddr());
            }else if msg.getMessage().getHeader() == "SendMsg"{
                println!("SendMsg");
               match Rooms.findRoomWithAddr(msg.getUser().getAddr()){
                    Ok(room) => {
                        println!("calling broadcast...");
                        println!("room {:?}",room);
                        let mut buff = bincode::serialize(&Message::new("Error".to_string(),"error finding room".to_string())).unwrap();
                        buff.resize(MSG_SIZE, 0);
                        room.broadcastToAll(Message::new(msg.getMessage().getHeader(),msg.getMessage().getData()))
                    },
                    Err(e) => println!("Error in SendMsg! {:?}", e),
               }
            }else{
                clients = clients.into_iter().filter_map(|mut client| {
                    println!("msg: {:?}", msg);
                    let mut buff = bincode::serialize(&msg.getMessage()).unwrap();
                    buff.resize(MSG_SIZE, 0);
                    println!("client {:?}", client);
                    client.write_all(&buff).map(|_| client).ok()
                }).collect::<Vec<_>>();
            }
            
        }

        sleep();
    }
}   