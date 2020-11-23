use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 128;

fn sleep() {
    thread::sleep(::std::time::Duration::from_millis(100));
}

#[path = "../../message.rs"]
mod message;
use crate::message::Message::Message;

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Listener failed to bind");
    server.set_nonblocking(true).expect("failed to initialize non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<Message>();
    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);

            let tx = tx.clone();
            clients.push(socket.try_clone().expect("failed to clone client"));

            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];

                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        println!("buff: {:?}",buff);
                        let msg = buff.into_iter().collect::<Vec<_>>();
                        println!("buff msg: {:?}",msg);
                        let xd: Message = bincode::deserialize(&msg).unwrap();
                        println!("important: {:?}", xd);    
                        println!("{}: {:?}", addr, msg);
                        tx.send(xd).expect("failed to send msg to rx");
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
            if msg.getHeader() == "JoinRoom" {
            }
            clients = clients.into_iter().filter_map(|mut client| {
                println!("msg: {:?}", msg);
                let mut buff = bincode::serialize(&msg).unwrap();
                buff.resize(MSG_SIZE, 0);

                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        sleep();
    }
}   