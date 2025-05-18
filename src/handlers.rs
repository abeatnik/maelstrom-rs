use crate::node::{ Node, MessageHandler };
use crate::message::{ Message, MessageBody };
use std::sync::{ Arc, Mutex };

use crate::broadcast::BroadcastContext;

pub struct InitHandler;

impl<C> MessageHandler<C> for InitHandler {
    fn handle(&self, node: &mut Node<C>, req: Message) -> Result<(), String> {
        match req.body {
            MessageBody::RequestInit { msg_id, node_id, node_ids } => {
                node.node_id = Some(node_id.clone());
                node.node_ids = node_ids.into_iter().map(Some).collect();
                node.next_msg_id += 1;

                node.send(req.src, MessageBody::ResponseInitOk {
                    in_reply_to: msg_id,
                    msg_id: node.next_msg_id,
                    node_id: node.node_id.clone().unwrap(),
                    node_ids: node.node_ids.clone().into_iter().flatten().collect(),
                });
                Ok(())
            }
            _ => Err("Invalid message for InitHandler".to_string()),
        }
    }
}

pub struct EchoHandler;

impl<C> MessageHandler<C> for EchoHandler {
    fn handle(&self, node: &mut Node<C>, req: Message) -> Result<(), String> {
        match req.body {
            MessageBody::RequestEcho { msg_id, echo } => {
                if node.node_id.as_deref() == Some(req.dest.as_str()) {
                    node.next_msg_id += 1;
                    node.send(req.src, MessageBody::ResponseEchoOk {
                        in_reply_to: msg_id,
                        echo,
                        msg_id: node.next_msg_id,
                    });
                }
                Ok(())
            }
            _ => Err("Invalid message for EchoHandler".to_string()),
        }
    }
}

pub struct TopologyHandler;

impl MessageHandler<Arc<BroadcastContext>> for TopologyHandler {
    fn handle(&self, node: &mut Node<Arc<BroadcastContext>>, req: Message) -> Result<(), String> {
        match req.body {
            MessageBody::RequestTopology { msg_id, topology } => {
                if let Some(id) = &node.node_id {
                    if let Some(n) = topology.get(id) {
                        let mut neighbors = node.ctx.neighbors.lock().unwrap();
                        *neighbors = n.clone();
                        node.log(format!("Neighbors set to: {:?}", neighbors));
                    }
                }
                node.next_msg_id += 1;
                node.send(req.src, MessageBody::ResponseTopologyOk {
                    msg_id: node.next_msg_id,
                    in_reply_to: msg_id,
                });

                Ok(())
            }
            _ => Err("Invalid message for TopologyHandler".to_string()),
        }
    }
}

pub struct BroadcastHandler;

impl MessageHandler<Arc<BroadcastContext>> for BroadcastHandler {
    fn handle(&self, node: &mut Node<Arc<BroadcastContext>>, req: Message) -> Result<(), String> {
        match req.body {
            MessageBody::RequestBroadcast { msg_id, message } => {
                let is_new = {
                    let mut messages = node.ctx.messages.lock().unwrap();
                    messages.insert(message)
                };

                if is_new {
                    let neighbors: Vec<String> = {
                        let guard = node.ctx.neighbors.lock().unwrap();
                        guard.clone()
                    };

                    for neighbor in neighbors {
                        let body = MessageBody::RequestBroadcast {
                            msg_id: None,
                            message,
                        };
                        node.send(neighbor, body);
                    }
                }
                let reply_msg_id = if msg_id.is_some() {
                    node.next_msg_id += 1;
                    Some(node.next_msg_id)
                } else {
                    None
                };

                node.send(req.src, MessageBody::ResponseBroadcastOk {
                    msg_id: reply_msg_id,
                    in_reply_to: msg_id,
                });
                Ok(())
            }
            _ => Err("Invalid message for BroadcastHandler".to_string()),
        }
    }
}

pub struct ReadHandler;

impl MessageHandler<Arc<BroadcastContext>> for ReadHandler {
    fn handle(&self, node: &mut Node<Arc<BroadcastContext>>, req: Message) -> Result<(), String> {
        match req.body {
            MessageBody::RequestRead { msg_id } => {
                node.next_msg_id += 1;
                let mut messages_vec: Vec<u32> = node.ctx.messages
                    .lock()
                    .unwrap()
                    .iter()
                    .copied()
                    .collect();
                messages_vec.sort_unstable();
                node.send(req.src, MessageBody::ResponseReadOk {
                    msg_id: node.next_msg_id,
                    in_reply_to: msg_id,
                    messages: messages_vec,
                });

                Ok(())
            }
            _ => Err("Invalid message for ReadHandler".to_string()),
        }
    }
}
