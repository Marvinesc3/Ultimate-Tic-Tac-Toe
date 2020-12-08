use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 128;
// const PORT: &str= ":6000";
// const PORT: &str = &*std::env::var("PORT").unwrap().to_owned();
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





fn main() {
    let PORT: &str = &*std::env::var("PORT").unwrap().to_owned();
    // let PORT: &str= "6000";
    // println!("here1?");
    let addr = Ipv4Addr::UNSPECIFIED;
    let IP = std::env::var("IP").unwrap().to_owned()
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
                    let mut buff = bincode::serialize(&Message::new("enteredRoom".to_string(),"data".to_string())).unwrap();
                    
                    let temp = msg.getUser().getStream().write_all(&buff);
                    println!("temp {:?}",temp);
                }else{
                    println!("did not find room");
                    let mut buff = bincode::serialize(&Message::new("Error".to_string(),"error finding room".to_string())).unwrap();
                    println!("buff {:?}",buff);
                    let mut temps = vec![];
                    temps.push(msg.getUser().getStream().try_clone().expect("failed to clone client"));
                    temps = temps.into_iter().filter_map(|mut temp| {
                        println!("msg: {:?}", msg);
                        let mut buff = bincode::serialize(&msg.getMessage()).unwrap();
                        buff.resize(MSG_SIZE, 0);
                        temp.write_all(&buff).map(|_| temp).ok()
                    }).collect::<Vec<_>>();
                    let temp = msg.getUser().getStream().write_all(&buff).map(|_| msg.getUser().getStream()).ok().unwrap();
                    println!("temp {:?}",temp);                    
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