use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{self, Write, BufRead};
use std::thread;
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};

use crate::message::{MessageBody, Message};

pub struct Node {
    node_id : Option<String>,
    next_msg_id : u32,
    node_ids: Vec<Option<String>>,

    handlers : Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn Fn(&mut Node, Message) -> Result<(), String> + Send>>>>>>,
    lock: Mutex<()>,      // synchronized message sending
    log_lock: Mutex<()>,  // synchronized logging
}


impl Node {
    pub fn new() -> Self {

        let mut node = Self {
            node_id : None,
            node_ids: vec![],
            next_msg_id : 0,
            handlers : Arc::new(Mutex::new(HashMap::new())),
            lock : Mutex::new(()),
            log_lock : Mutex::new(()),
        };

        node.add_init_handler();
        node.add_echo_handler();

        node
    }

    fn log(&self, message: String){
        let mut stderr = io::stderr();
        let _lock = self.log_lock.lock().unwrap();
        writeln!(stderr,"{}", message).unwrap();
    }

    fn send(&self, dest: String, body: MessageBody){
        let mut stdout= io::stdout(); 
        let _lock = self.lock.lock().unwrap();
        let message = json!({
                "src": self.node_id,
                "dest": dest,
                "body": body,
            });
        let json_string = serde_json::to_string(&message).unwrap();
        writeln!(stdout, "{}", json_string).unwrap();
        stdout.flush().unwrap();

    }

    fn on<F>(&mut self, message_type: String, handler: F)
    where
        F: Fn(&mut Node, Message) -> Result<(), String> + Send + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(message_type, Arc::new(Mutex::new(Box::new(handler))));

    }

    fn handle(&self, message_type: String)-> Option<Arc<Mutex<Box<dyn Fn(&mut Node, Message) -> Result<(), std::string::String> + Send>>>>{
        let handlers = self.handlers.lock().unwrap();
        if let Some(handler) =  handlers.get(&message_type){
            let _lock = self.lock.lock().unwrap(); // ensures that only one thread logs at a time
            Some(handler.clone())
        } else {
            self.log(format!("Handler does not exist for message_type: {}", message_type));
            None
        }
    }

    fn add_init_handler(&mut self) {
        self.on("init".to_string(), | node: &mut Node, req: Message|{
            match req.body {
                MessageBody::RequestInit { msg_id, node_id, node_ids } => {
                    node.node_id = Some(node_id.clone());
                    node.node_ids = node_ids.into_iter().map(Some).collect();
                    node.log(format!("Initialized node {}", node_id));
                    node.next_msg_id += 1;
                    let body = MessageBody::ResponseInitOk{ 
                            in_reply_to: msg_id, 
                                msg_id: node.next_msg_id,  
                                node_id:  node.node_id.clone().unwrap(), 
                                node_ids : node.node_ids.clone().into_iter().filter_map(|x| x).collect(),
                            };
                    node.send(req.src, body);
                    Ok(())
                },
                _ => {
                    node.log(format!("matching on wrong request type"));
                    Err("matching on wrong request type".to_string())
                },
            }
        });
    }

    fn add_echo_handler(&mut self){
        self.on("echo".to_string(), |node: &mut Node, req: Message|{
            match req.body {
                MessageBody::RequestEcho { msg_id, echo } => {
                    if Some(req.dest.as_str()) == node.node_id.as_deref(){
                        node.log(format!("Echoing: {}", &echo));
                        node.next_msg_id +=1;
                        let body =  MessageBody::ResponseEchoOk { 
                                in_reply_to: msg_id, 
                                echo,
                                msg_id: node.next_msg_id, 
                                };
                        node.send(req.src, body);
                    } 
                    Ok(())
                },
                _ => {
                    node.log(format!("matching on wrong request type"));
                    Err("matching on wrong request type".to_string())
                },
            }
        });

    }

    pub fn main(&mut self){
        let mut stdin = io::stdin();

        for line in stdin.lock().lines(){
            match line {
                Ok(input) => {
                    match serde_json::from_str::<Message>(&input){
                        Ok(request) => {
                            let msg_type = request.msg_type().to_string();
                            if let Some(handler) = self.handle(msg_type.clone()){
                                handler.lock().unwrap()(self, request).expect(format!("Failed to process request {:?}", &msg_type).as_str())
                            }
                        },
                        Err(e) => {
                            self.log(format!("Error parsing request: {}", e));
                        }

                    }
                }
                Err(e) => {
                    self.log(format!("Error reading line: {}", e));
                }
            }
        }

    }
        
            
}


