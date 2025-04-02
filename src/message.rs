use serde::{Deserialize,Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub src : String,
    pub dest : String,
    pub body : MessageBody,
}


impl Message {
    pub fn msg_type(&self) -> &str {
        match &self.body {
            MessageBody::Init { .. } => "init",
            MessageBody::Echo { .. } => "echo",  
            MessageBody::Topology { .. } => "topology",
            MessageBody::Read { ..} => "read",
            MessageBody::Broadcast { ..} => "broadcast",
            MessageBody::InitOk { .. } => "init_ok",
            MessageBody::EchoOk { .. } => "echo_ok",
            MessageBody::TopologyOk {.. } => "topology_ok",
            MessageBody::ReadOk { .. } => "read",
            MessageBody::BroadcastOk {.. } => "broadcast_ok",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MessageBody {
    #[serde(rename = "init")]
    Init {
        msg_id : u64,
        node_id: String,
        #[serde(default)] 
        node_ids: Vec<String>,
    },

    #[serde(rename = "echo")]
    Echo {
        msg_id : u64,
        echo: String,
    },

    #[serde(rename = "topology")]
    Topology {
        msg_id : u64,
        topology : HashMap<String, Vec<String>>,
    },

    #[serde(rename="read")]
    Read {
        msg_id : u64,
    },
    
    #[serde(rename = "broadcast")]
    Broadcast {
        msg_id : Option<u64>,
        message : u64,
    },

    #[serde(rename = "init_ok")]
    InitOk {
        msg_id : u64,
        node_id: String,
        #[serde(default)] 
        node_ids: Vec<String>,
        in_reply_to: u64,
    },

    #[serde(rename = "echo_ok")]
    EchoOk {
        msg_id : u64,
        in_reply_to: u64,
        echo: String,
    },

    #[serde(rename = "topology_ok")]
    TopologyOk {
        msg_id : u64,
        in_reply_to: u64,
    },

    #[serde(rename="read_ok")]
    ReadOk {
        msg_id : u64,
        messages : Vec<u64>,
        in_reply_to: u64,
    },

    #[serde(rename = "broadcast_ok")]
    BroadcastOk {
        msg_id : u64,
        in_reply_to: u64,
    }
}


