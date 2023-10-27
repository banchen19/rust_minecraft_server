use std::{collections::hash_map::RandomState};

use rust_raknet::{RaknetSocket, Reliability};

pub struct DataPacket{
    pub isEncoded:bool,
    pub channel:u32,
    pub buf:Vec<u8>,
    pub reliability:Reliability
}

impl DataPacket {
    pub fn new(buf:Vec<u8>)->Self{
        DataPacket { 
            isEncoded:false,
            channel:0,
            reliability:Reliability::ReliableOrdered,
            buf,
        }
    }
}