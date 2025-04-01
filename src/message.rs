use serde::{Deserialize,Serialize};


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
            MessageBody::ResponseInitOk { .. } => "init_ok",
            MessageBody::ResponseEchoOk { .. } => "echo_ok",
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
}