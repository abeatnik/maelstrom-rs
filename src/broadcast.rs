use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use crate::message::{MessageBody, Message};
use crate::node::Node;

pub struct Broadcast{
    pub node: Node,
    pub neighbors: Arc<Mutex<Vec<String>>>,
    messages : Arc<Mutex<HashSet<u64>>>,
}

impl Broadcast {
    pub fn new() -> Self {
        let mut broadcast = Self{
            node: Node::new(),
            neighbors: Arc::new(Mutex::new(vec![])),
            messages : Arc::new(Mutex::new(HashSet::new())),
        };
        broadcast.add_topology_handler();
        broadcast.add_read_handler();
        broadcast.add_broadcast_handler();

        broadcast
    }

    fn add_topology_handler(&mut self){
        let neighbors_clone = Arc::clone(&self.neighbors);

        self.node.on("topology".to_string(), move |node: &mut Node, req: Message|{
            match req.body {
                MessageBody::Topology { msg_id, topology } => {
                    let mut locked_neighbors = neighbors_clone.lock().unwrap();
                    if let Some(neighbors) = node.node_id.as_ref().and_then(|id| topology.get(id)){
                        *locked_neighbors = neighbors.to_vec();
                    }
                    node.log(format!("my neighbors are: {:?}", locked_neighbors));
                    let new_msg_id = node.increased_next_msg_id_and_return();
                    let body = MessageBody::TopologyOk { 
                        msg_id: new_msg_id,
                        in_reply_to: msg_id 
                        };
                    node.send(req.src, body);
                    Ok(())
                },
                _ => Err("not ok".to_string()),
            }});
    }

    fn add_read_handler(&mut self){
        let messages_clone = Arc::clone(&self.messages);

        self.node.on("read".to_string(), move|node: &mut Node, req: Message|{
            match req.body {
                MessageBody::Read { msg_id } =>
                    {
                    let locked_messages = messages_clone.lock().unwrap();
                    let mut messages_vec: Vec<u64> = locked_messages.iter().cloned().collect(); 
                    messages_vec.sort();
                    let new_msg_id = node.increased_next_msg_id_and_return();
                    let body = MessageBody::ReadOk { 
                        msg_id: new_msg_id,
                        messages : messages_vec,
                        in_reply_to: msg_id 
                        };
                    node.send(req.src, body);
                    Ok(())
                },
                _ => Err("not ok".to_string()),
            }
        });
    }

    fn add_broadcast_handler(&mut self){
        let messages_clone = Arc::clone(&self.messages);
        let neighbors_clone = Arc::clone(&self.neighbors);
        self.node.on("broadcast".to_string(), move|node: &mut Node, req: Message|{
            match req.body {
                MessageBody::Broadcast { msg_id, message } => {
                    let mut locked_messages = messages_clone.lock().unwrap();
                    if !locked_messages.contains(&message){
                        let locked_neighbors = neighbors_clone.lock().unwrap();
                        locked_messages.insert(message);
                        for neighbor in locked_neighbors.iter(){
                            let body = MessageBody::Broadcast { message, msg_id : None };
                            node.send(neighbor.to_string(), body);
                        }
                    }
                    match msg_id {
                        Some(m) => {
                            let new_msg_id = node.increased_next_msg_id_and_return();
                            let body = MessageBody::BroadcastOk { 
                                msg_id: new_msg_id,
                                in_reply_to: m
                                };
                            node.send(req.src, body);
                            Ok(())
                        },
                        None => {Ok(())}
                    }
                },
                _ => Err("not ok".to_string()),
            }
        });
    }



}

