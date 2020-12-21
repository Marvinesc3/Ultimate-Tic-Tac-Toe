#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_snake_case)]



pub mod TicTacToeStructs {
    
    use std::io::{ErrorKind, Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread;
    use std::net::SocketAddr;
    use serde::{Deserialize, Serialize};
    const MSG_SIZE: usize = 128;
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
    #[derive(Clone, Debug)]
    pub struct ClientUser{
        roomid: String,
        id: String,
    }

    impl ClientUser {
        pub fn new() -> ClientUser{
            ClientUser{roomid: "".to_string(),id: "".to_string()}
        }
        pub fn setRoomId(&mut self, roomid:String){
            self.roomid = roomid;
        }
        pub fn setId(&mut self, id:String){
            self.id = id;
        }pub fn getId(&mut self)-> String{
            self.id.clone()
        }pub fn getRoomId(&mut self)-> String{
            self.roomid.clone()
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
        playersTurn: i8,
        // Protocols = vec!["JoinRoom".to_string(),"PressBtn".to_string()],
    }
    
    impl Room {
        pub fn new(id: String) -> Room {
            let users: Vec<User> = vec![];
            Room {users, id , playersTurn: 1}
        }

        pub fn Move(&mut self, msg:String){
            let mut index = 0;
            if self.playersTurn == 1 { index = 2} else{ index = 1}
            println!("index {}",(index-1));
            let user = &self.users[(index-1)as usize];
            let mut buff = bincode::serialize(&Message::new("EnemyMove".to_string(),
                msg)).unwrap();
            let mut temp = user.getStream().try_clone().expect("failed to clone client");
            buff.resize(MSG_SIZE, 0);
            if let Err(io_error) = temp.write_all(&buff) {
                println!("io error {}", io_error);
            }
            self.playersTurn = index;

        }

        pub fn getNumberOfPlayers(&self)->i8{
            self.users.len() as i8
        }
        pub fn getPlayersTurn(&self)->i8{
            self.playersTurn
        }
        pub fn addMemeber(&mut self, user: User)-> bool{
            if let Some(pos) = self.users.iter().position(|x| x.addr == user.addr){
                return false
            }   
            self.users.push(user);
            true
        }
        pub fn removeMemeber(&mut self, user:User){
            self.users.retain(|x| x.getAddr() != user.getAddr());
        }
        pub fn broadcastToAll(&self, msg: Message){
            println!("calling broadcast... in object");
            println!("users {:?}", self.users);
           let temp =  self.users.iter().filter_map(|client| {
                println!("msg: {:?}", msg);
                let mut buff = bincode::serialize(&msg).unwrap();
                buff.resize(MSG_SIZE, 0);
                
                client.getStream().try_clone().expect("failed to clone client").write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        pub fn getId(self)-> String{
            self.id
        }
        pub fn isAddrInRoom(&self, addr:SocketAddr)-> bool{
            if let Some(pos) = self.users.iter().position(|x| x.addr == addr){
                return true
            }   
            false
        }
        pub fn removeSocket(&mut self, addr:SocketAddr){
            self.users.retain(|x| x.getAddr() != addr);
        }
        pub fn startGame(&self, addr: SocketAddr){
            let user = &self.users[0];
            let mut buff = bincode::serialize(&Message::new("PlayerTurn".to_string(),
            "".to_string())).unwrap();
            let mut temp = user.getStream().try_clone().expect("failed to clone client");
            buff.resize(MSG_SIZE, 0);
            if let Err(io_error) = temp.write_all(&buff) {
                println!("io error {}", io_error);
            }
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
            let mut temp =  self.rooms.iter().find(|&x| x.id == id);
            temp
        } 

        pub fn addRoom(&mut self, id:String) -> &Room{
            let x : Room = Room::new(id);
            self.rooms.push(x);
            &self.rooms[self.rooms.len()-1]
        }
        pub fn addMemberToRoom(&mut self, id:String, user: User) -> bool{
            if let Some(pos) = self.rooms.iter().position(|x| x.id == id){
                if self.rooms[pos].getNumberOfPlayers() == 2{
                     return false;
                }
                let added = self.rooms[pos].addMemeber(user);
                println!("room: {:?} added: {:?}",self.rooms,added );
                return added
            }   
            println!("not found :( poggers");
            false
        }
        pub fn findRoomWithAddr(&mut self,addr: SocketAddr) -> Result<&Room, &'static str>{
            if let Some(pos) = self.rooms.iter().position(|room| room.isAddrInRoom(addr) == true){
                return Ok(&self.rooms[pos])
            }  
            Err("Addr not found in any rooms!")
        }
        pub fn MoveWithAddr(&mut self,msg:String, addr: SocketAddr){
            if let Some(pos) = self.rooms.iter().position(|room| room.isAddrInRoom(addr) == true){
                let z = &mut self.rooms[pos];
                z.Move(msg);
            }  
        }
        pub fn removeSocket(&mut self, addr: SocketAddr) {
           if let Some(pos) = self.rooms.iter().position(|room| room.isAddrInRoom(addr) == true){
                self.rooms[pos].removeSocket(addr);
            }  
        }
        pub fn startGame(&mut self, addr: SocketAddr){
            if let Some(pos) = self.rooms.iter().position(|room| room.isAddrInRoom(addr) == true){
                self.rooms[pos].startGame(addr);
            } 
        }
    }

    #[derive(Clone, Serialize, Deserialize, Debug)]
    pub struct TicTacToeBoard{
        boardState: [[i8; 9]; 9] ,
        innerBoardsState: [[i8; 3]; 3],
        playersTurn : bool,
        winner :i8,
    } 
    impl TicTacToeBoard{
        pub fn new()-> TicTacToeBoard {
            TicTacToeBoard{boardState: [[0; 9]; 9] ,innerBoardsState: [[0; 3]; 3], playersTurn: false, winner:0}
        }
        pub fn PlaceNextMarker(&mut self, x: usize,y:usize )-> bool{
            if self.winner == 0 && self.innerBoardsState[x/3][y/3] == 0 && self.boardState[x][y] == 0 {
                //place
                self.boardState[x][y] = if self.playersTurn { 1 } else { 2 };
                self.playersTurn = !self.playersTurn;
                self.winner == self.checkBoardWinner();

                return true
            }else {
                println!("else");
                return false
            }
        }

        pub fn getWinner(&self) -> i8{
            self.winner
        }

        pub fn Innerhas(&self, x: usize, y: usize, player: i8) -> bool{
            self.boardState[x][y] == player 
        }

        pub fn checkBoardWinner(&mut self)->i8{
            //self.boardStateX[x%3*3+1] == 0 && self.boardStateY[y/3*3+1] == 0
            for player in 1..3 {
                for i in 0..self.innerBoardsState.len() {
                    // println!("x: {}, y:{}",i%3*3+1, i/3*3+1 );
                    self.checkInnerBoard(i%3*3+1-1, i/3*3+1-1, player);
                }
                
            }
            return self.checkBoard();
        }

        pub fn Outerhas(&self, x: usize, y: usize, player: i8) -> bool{
            self.boardState[x][y] == player 
        }

        pub fn checkBoard(&self) ->i8{
            for player in 1..3 {
                for dx in 0..3 {
                    if self.Outerhas(dx, 0,player) && self.Outerhas(dx, 1,player) &&self.Outerhas(dx, 2,player){
                        println!("Winner horizontal x:{} ",dx);
                        return player;
                    }
                }
                for dy in 0..3 {
                    if self.Outerhas(0, dy,player) && self.Outerhas(1, dy,player) &&self.Outerhas(2, dy,player){
                        println!("Winner vetrical y:{} ",dy);
                        return player;
                    }
                }
                if self.Outerhas(0, 0,player) && self.Outerhas(1, 1,player) &&self.Outerhas(2, 2,player){
                    println!("Winner diagonally t->b");
                    return player;
                }
                if self.Outerhas(2, 0,player) && self.Outerhas(1, 1,player) &&self.Outerhas(0, 2,player){
                    println!("Winner diagonally b->t");
                    return player;
                }
            }
            return 0;
        }

        pub fn checkInnerBoard(&mut self, x:usize, y:usize, player: i8){
            for dx in 0..3 {
                if self.Innerhas(x+dx, y,player) && self.Innerhas(x+dx, y+1,player) &&self.Innerhas(x+dx, y+2,player){
                    println!("horizontal x:{} ",dx+x);
                    self.innerBoardsState[x/3][y/3]  = player;
                }
            }
            for dy in 0..3 {
                if self.Innerhas(x, y+dy,player) && self.Innerhas(x+1, y+dy,player) &&self.Innerhas(x+2, y+dy,player){
                    println!("vetrical y:{} ",dy+y);
                    self.innerBoardsState[x/3][y/3]  = player;
                }
            }
            if self.Innerhas(x, y,player) && self.Innerhas(x+1, y+1,player) &&self.Innerhas(x+2, y+2,player){
                println!("diagonally t->b");
                self.innerBoardsState[x/3][y/3] = player;
            }
            if self.Innerhas(x+2, y,player) && self.Innerhas(x+1, y+1,player) &&self.Innerhas(x, y+2,player){
                println!("diagonally b->t");
                self.innerBoardsState[x/3][y/3]  = player;
            }
        }       
        pub fn test(&self){
            // println!("board {:?}")
        }
    }
} 
