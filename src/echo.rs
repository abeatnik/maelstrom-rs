use std::io::{self, BufRead, Stderr, Stdout, Write};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};


#[derive(Debug, Deserialize, Serialize)]
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


#[derive(Debug, Deserialize, Serialize)]
struct Body {
    msg_id : u32,
    #[serde(rename = "type")]
    msg_type: MessageType,
    node_id: Option<String>,
    #[serde(default)] 
    node_ids: Vec<Option<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ReplyBody {
    #[serde(flatten)]
    body: Body,
    in_reply_to : u32,
}

#[derive(Debug, Deserialize, Serialize)]
enum Response {
    InitReply {
        src: String,
        dest: String,
        body : ReplyBody, // I would like here to only allow Bodies whose msg_type field contains MessageType::InitOk, but I don't know how to enforce that...
    },
    UnknownReply {error: String},
}

impl Response {
    fn send(&self) {
        let mut stdout: Stdout = io::stdout(); 
        if let Response::InitReply { src, dest, body } = self {
            let message = json!({
                "src": src,
                "dest": dest,
                "body": body,
            });
            let json_string = serde_json::to_string(&message).unwrap();
            writeln!(stdout, "{}", json_string).unwrap();
            stdout.flush().unwrap();
        }
        
        
    }
}


pub struct EchoServer {
    node_id : Option<String>,
    next_msg_id : u32,
    node_ids: Vec<Option<String>>
}

impl EchoServer {

    pub fn new() -> Self {
        Self {
            node_id : None,
            node_ids: vec![],
            next_msg_id : 0,
        }
    }

    fn handle_request(&mut self, request: Request){
        let mut stderr = io::stderr();

        writeln!(stderr, "Received {:?}", request).expect("Failed to write to stderr");

        match request.body.msg_type {
            MessageType::Init => {
                if let Some(node_id) = request.body.node_id {
                    self.node_id = Some(node_id.clone());
                    self.node_ids = request.body.node_ids.clone();
                    writeln!(stderr, "Initialized node {}", node_id).expect("Failed to initialize node");
                    self.next_msg_id += 1;
                    let init_response = Response::InitReply {
                        src : request.dest.unwrap(),
                        dest : request.src.unwrap(),
                        body : ReplyBody { in_reply_to: request.body.msg_id, body : Body {msg_id: self.next_msg_id, msg_type: MessageType::InitOk, node_id: self.node_id.clone(), node_ids : self.node_ids.clone()}}
                    };
                    
                    init_response.send();
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