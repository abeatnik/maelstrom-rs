use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::{self, Write, BufRead};
use std::thread;
use serde::{Deserialize,Serialize};
use serde_json::{Value,json};

use crate::message::{MessageBody, Message};
use crate::node::Node;

pub struct Broadcast{
    pub node: Node,
    pub neighbors: Arc<Mutex<Vec<String>>>,
}

impl Broadcast {
    pub fn new() -> Self {
        let mut broadcast = Self{
            node: Node::new(),
            neighbors: Arc::new(Mutex::new(vec![])),
        };
        broadcast.add_topology_handler();

        broadcast
    }

    fn add_topology_handler(&mut self){
        let neighbors_clone = Arc::clone(&self.neighbors);

        self.node.on("topology".to_string(), move |node: &mut Node, req: Message|{
            match req.body {
                MessageBody::RequestTopology { msg_id, topology } => {
                    let mut locked_neighbors = neighbors_clone.lock().unwrap();
                    if let Some(neighbors) = node.node_id.as_ref().and_then(|id| topology.get(id)){
                        *locked_neighbors = neighbors.to_vec();
                    }
                    node.log(format!("my neighbors are: {:?}", locked_neighbors));
                    node.next_msg_id += 1;
                    let body = MessageBody::ResponseTopologyOk { 
                        msg_id: node.next_msg_id,
                        in_reply_to: msg_id 
                        };
                    node.send(req.src, body);
                    Ok(())
                },
                _ => Err("not ok".to_string()),
            }});
    }



}

