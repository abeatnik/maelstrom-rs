use std::collections::HashSet;
use std::sync::{ Arc, Mutex };

use crate::node::Node;
use crate::handlers::{ BroadcastHandler, TopologyHandler, ReadHandler };

#[derive(Default)]
pub struct BroadcastContext {
    pub neighbors: Mutex<Vec<String>>,
    pub messages: Mutex<HashSet<u32>>,
}

pub struct Broadcast {
    pub ctx: Arc<BroadcastContext>,
    pub node: Node<Arc<BroadcastContext>>,
}

impl Broadcast {
    pub fn new() -> Self {
        let ctx = Arc::new(BroadcastContext::default());

        let mut node = Node::new(ctx.clone());
        node.register_handler("topology", TopologyHandler {});
        node.register_handler("broadcast", BroadcastHandler {});
        node.register_handler("read", ReadHandler {});

        Self { ctx, node }
    }

    pub fn main_loop(&mut self) {
        self.node.main_loop();
    }
}
