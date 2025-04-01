use std::io::{self, BufRead, Stderr, Stdout, Write};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};


#[derive(Debug, Deserialize)]
struct Message {
    src : String,
    dest : String,
    body : MessageBody,
}

impl Message {
    fn send(&self) {
        let mut stdout: Stdout = io::stdout(); 
            let message = json!({
                "src": self.src,
                "dest": self.dest,
                "body": self.body,
            });
            let json_string = serde_json::to_string(&message).unwrap();
            writeln!(stdout, "{}", json_string).unwrap();
            stdout.flush().unwrap();
    }
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
enum MessageBody {
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

    fn handle_request(&mut self, request: Message){
        let mut stderr = io::stderr();

        writeln!(stderr, "Received {:?}", request).expect("Failed to write to stderr");
            match request.body {
                MessageBody::RequestInit {msg_id,node_id, node_ids} => {
                        self.node_id = Some(node_id.clone());
                        self.node_ids = node_ids.into_iter().map(Some).collect();
                        writeln!(stderr, "Initialized node {}", node_id).expect("Failed to write to stderr");
                        self.next_msg_id += 1;
                        let init_response = Message{
                            src : request.dest,
                            dest : request.src,
                            body : MessageBody::ResponseInitOk{ 
                                in_reply_to: msg_id, 
                                    msg_id: self.next_msg_id,  
                                    node_id:  self.node_id.clone().unwrap(), 
                                    node_ids : self.node_ids.clone().into_iter().filter_map(|x| x).collect(),
                                }
                        };
                        init_response.send();
                    },
                MessageBody::RequestEcho {msg_id, echo} => {
                        if Some(request.dest.as_str()) == self.node_id.as_deref(){
                            writeln!(stderr, "Echoing: {}", &echo).expect("Failed to write to stderr");
                            self.next_msg_id +=1;
                            let echo_response = Message{
                                src : request.dest.clone(),
                                dest : request.src.clone(),
                                body : MessageBody::ResponseEchoOk { 
                                    in_reply_to: msg_id, 
                                    echo,
                                    msg_id: self.next_msg_id, 
                                    }
                            };
                            echo_response.send();
                        }
                },
                _ => {
                writeln!(stderr, "Request handling not implemented for unknown type").expect("Failed to initialize node");
                },
            }
        }
    


    pub fn main(&mut self){
        let stdin = io::stdin();

        for line in stdin.lock().lines(){
            match line {
                Ok(input) => {
                    match serde_json::from_str::<Message>(&input){
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
