use std::io::{self, BufRead, Write};
use serde::{Deserialize,Serialize};
use serde_json::Value;


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Init,
    InitOk,

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Deserialize)]
struct Request {
    src : Option<String>,
    dest : Option<String>,
    body : Body,
}


#[derive(Debug, Deserialize)]
struct Body {
    msg_id : u32,
    #[serde(rename = "type")]
    msg_type: MessageType,
    node_id: Option<String>,
    node_ids: Vec<Option<String>>,
}


#[derive(Debug, Deserialize)]
enum Response {
    InitReply {
        src: String,
        dest: String,
        body : Body, // I would like here to only allow Bodies whose msg_type field contains MessageType::InitOk, but I don't know how to enforce that...
    },
    UnknownReply {error: String},
}


pub struct EchoServer {
    node_id : Option<String>,
}

impl EchoServer {

    pub fn new() -> Self {
        Self {
            node_id : None,
        }
    }

    fn handle_request(&mut self, request: Request){
        let mut stderr = io::stderr();

        writeln!(stderr, "Received {:?}", request).expect("Failed to write to stderr");

        match request.body.msg_type {
            MessageType::Init => {
                if let Some(node_id) = request.body.node_id {
                    self.node_id = Some(node_id.clone());
                    writeln!(stderr, "Initialized node {}", node_id).expect("Failed to initialize node");
                }
            },
            MessageType::Unknown => {
                writeln!(stderr, "Ignoring unknown message type: {:?}", request.body.msg_type).expect("Failed to initialize node");
            },
            _ => {
                writeln!(stderr, "Request handling not implemented for message type: {:?}", request.body.msg_type).expect("Failed to initialize node");
            }
        }
    }



    pub fn main(&mut self){
        let stdin = io::stdin();

        for line in stdin.lock().lines(){
            match line {
                Ok(input) => {
                    match serde_json::from_str::<Request>(&input){
                        Ok(request) => self.handle_request(request),
                        Err(e) => {
                            writeln!(io::stderr(), "Error reading line: {}", e).expect("Failed to write to stderr");
                        }

                    }
                }
                Err(e) => {
                    writeln!(io::stderr(), "Error reading line: {}", e).expect("Failed to write to stderr");
                }
            }
        }
    }
}