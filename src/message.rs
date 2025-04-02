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
            MessageBody::RequestInit { .. } => "init",
            MessageBody::RequestEcho { .. } => "echo", 
            MessageBody::RequestTopology { .. } => "topology",
            MessageBody::ResponseInitOk { .. } => "init_ok",
            MessageBody::ResponseEchoOk { .. } => "echo_ok",
            MessageBody::ResponseTopologyOk {.. } => "topology_ok",
        }
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum MessageBody {
    #[serde(rename = "init")]
    RequestInit {
        msg_id : u32,
        node_id: String,
        #[serde(default)] 
        node_ids: Vec<String>,
    },

    #[serde(rename = "echo")]
    RequestEcho {
        msg_id : u32,
        echo: String,
    },

    #[serde(rename = "topology")]
    RequestTopology {
        msg_id : u32,
        topology : HashMap<String, Vec<String>>,
    },

    #[serde(rename = "init_ok")]
    ResponseInitOk {
        msg_id : u32,
        node_id: String,
        #[serde(default)] 
        node_ids: Vec<String>,
        in_reply_to: u32,
    },

    #[serde(rename = "echo_ok")]
    ResponseEchoOk {
        msg_id : u32,
        in_reply_to: u32,
        echo: String,
    },

    #[serde(rename = "topology_ok")]
    ResponseTopologyOk {
        msg_id : u32,
        in_reply_to: u32,
    }
}


