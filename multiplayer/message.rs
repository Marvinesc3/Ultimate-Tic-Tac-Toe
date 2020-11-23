#[allow(unused_variables)]
#[allow(dead_code)]
#[allow(non_snake_case)]

pub mod Message {
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
}