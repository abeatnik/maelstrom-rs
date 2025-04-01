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
}

impl Broadcast {
    pub fn new() -> Self {
        Self{
            node: Node::new(),
        }
    }
}

