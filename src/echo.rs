use std::io::{self, BufRead, Stderr, Stdout, Write};
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};


#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Init,
    InitOk,
    Echo,
    EchoOk,
    #[serde(other)]
    Unknown,
}

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
struct Body {
    msg_id : u32,
    #[serde(rename = "type")]
    msg_type: MessageType,
    node_id: String,
    #[serde(default)] 
    node_ids: Vec<String>,
}


#[derive(Debug, Deserialize, Serialize)]
enum RequestBody {
    Init {
        #[serde(flatten)]
        body: Body,
    },
    Echo {
        #[serde(flatten)]
        body: Body,
        echo : String,
    },
}


#[derive(Debug, Deserialize, Serialize)]
enum ReplyBody {
    InitOk {
        #[serde(flatten)]
        body: Body,
        in_reply_to : u32,
    },
    EchoOk {
        #[serde(flatten)]
        body: Body,
        in_reply_to : u32,
        echo : String,
    },
}


#[derive(Debug, Deserialize, Serialize)]
enum MessageBody {
    Request(RequestBody),
    Response(ReplyBody),
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

        if let MessageBody::Request(req) = request.body {
            match req {
                RequestBody::Init {body} => {
                    if body.msg_type == MessageType::Init {
                        self.node_id = Some(body.node_id.clone());
                        self.node_ids = body.node_ids.into_iter().map(Some).collect();
                        writeln!(stderr, "Initialized node {}", body.node_id).expect("Failed to write to stderr");
                        self.next_msg_id += 1;
                        let init_response = Message{
                            src : request.dest,
                            dest : request.src,
                            body : MessageBody::Response(ReplyBody::InitOk{ 
                                in_reply_to: body.msg_id, 
                                body: Body {
                                    msg_id: self.next_msg_id, 
                                    msg_type: MessageType::InitOk, 
                                    node_id:  self.node_id.clone().unwrap(), 
                                    node_ids : self.node_ids.clone().into_iter().filter_map(|x| x).collect(),
                                    }
                                })
                        };
                        init_response.send();
                    }
                },
                RequestBody::Echo {body, echo} => {
                        if Some(request.dest.as_str()) == self.node_id.as_deref(){
                            writeln!(stderr, "Echoing: {}", &echo).expect("Failed to write to stderr");
                            self.next_msg_id +=1;
                            let echo_response = Message{
                                src : request.dest.clone(),
                                dest : request.src.clone(),
                                body : MessageBody::Response(ReplyBody::EchoOk{ 
                                    in_reply_to: body.msg_id, 
                                    echo,
                                    body: Body {
                                        msg_id: self.next_msg_id, 
                                        msg_type: MessageType::EchoOk, 
                                        node_id: self.node_id.clone().unwrap(), 
                                        node_ids : self.node_ids.clone().into_iter().filter_map(|x| x).collect(),
                                        }
                                    })
                            };
                            echo_response.send();
                        } else {
                            writeln!(stderr, "Cannot echo message type: {:?}", body.msg_type).expect("Failed to write to stderr");
                        }
                },
                _ => {
                writeln!(stderr, "Request handling not implemented for unknown type").expect("Failed to initialize node");
                },
            }
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