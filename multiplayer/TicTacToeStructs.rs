#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_snake_case)]



pub mod TicTacToeStructs {
    use std::io::{ErrorKind, Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread;
    use serde::{Deserialize, Serialize};
    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct Message {
        header: String,
        data: String,
        // Protocols = vec!["JoinRoom".to_string(),"PressBtn".to_string()],
    }
    impl Message {
        
        pub fn getData(&self) -> String { 
            self.data.clone()          
        }
        pub fn getHeader(&self) -> String {
            self.header.clone()
        }

        pub fn new(header: String, data: String) -> Message {
            Message { header, data }
        }

        
    }
    
    
    #[derive(Debug)]
    pub struct ServerMessage {
        user:User,
        message: Message,
        // Protocols = vec!["JoinRoom".to_string(),"PressBtn".to_string()],
    }
    impl ServerMessage {
        
        pub fn getUser(&self) -> User { 
            self.user.clone()          
        }
        pub fn getMessage(&self) -> Message {
            self.message.clone()
        }

        pub fn new(user: User, message: Message) -> ServerMessage {
            ServerMessage { user, message }
        }

        
    }



    #[derive(Debug)]
    pub struct User {
        stream: std::net::TcpStream,
        addr: std::net::SocketAddr
    }

   impl User{ 
       pub fn new(stream: std::net::TcpStream<>, addr: std::net::SocketAddr) -> User{
            User{stream, addr}
       }
       pub fn getStream(&self) -> std::net::TcpStream{
            self.stream.try_clone().expect("failed to clone client")
       }
       pub fn getAddr(&self) -> std::net::SocketAddr{
            self.addr
       }
       pub fn clone(&self)->User{
            User{stream: self.stream.try_clone().expect("failed to clone stream"),addr: self.addr.clone()}
       }
   }

    #[derive( Debug)]
    pub struct Room {
        users: Vec<User>,
        id: String,
        // Protocols = vec!["JoinRoom".to_string(),"PressBtn".to_string()],
    }
    
    impl Room {
        pub fn new(id: String) -> Room {
            let users: Vec<User> = vec![];
            Room {users, id }
        }

        pub fn addMemeber(&mut self, user: User){
            self.users.push(user);
        }
        pub fn removeMemeber(&mut self, user:User){
            self.users.retain(|x| x.getAddr() != user.getAddr());
        }
        pub fn broadcastToAll(self, msg: Message){
            let temp = self.users.into_iter().filter_map(|client| {
                println!("msg: {:?}", msg);
                let mut buff = bincode::serialize(&msg).unwrap();
                // buff.resize(MSG_SIZE, 0);
                
                client.getStream().write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();

        }
        pub fn getId(self)-> String{
            self.id
        }
    } 

    #[derive( Debug)]
    pub struct Rooms {
        rooms: Vec<Room>,
        // Protocols = vec!["JoinRoom".to_string(),"PressBtn".to_string()],
    }
    
    impl Rooms {
        pub fn new() -> Rooms {
            let rooms: Vec<Room> = vec![];
            Rooms {rooms }
        }

        pub fn findRoom(&mut self, id:String) -> Option<&Room>{
            let temp = self.rooms.iter().find(|&x| x.id == id);
            temp
        } 

        pub fn addRoom(&mut self, id:String) -> &Room{
            let x : Room = Room::new(id);
            self.rooms.push(x);
            &self.rooms[self.rooms.len()-1]
        }
        pub fn addMemberToRoom(&mut self, id:String, user: User) -> bool{
            if let Some(pos) = self.rooms.iter().position(|x| x.id == id){
                
                self.rooms[pos].addMemeber(user);
                println!("room: {:?}",self.rooms );
                return true
            }   
            println!("not found :( poggers");
            false
        }
    } 
}