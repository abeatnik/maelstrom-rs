use std::collections::HashMap;
use std::sync::{ Arc, Mutex };
use std::io::{ self, Write, BufRead };
use serde_json::json;

use crate::message::{ Message, MessageBody };
use crate::handlers::{ InitHandler, EchoHandler };

pub trait MessageHandler<C>: Send + Sync {
    fn handle(&self, node: &mut Node<C>, msg: Message) -> Result<(), String>;
}

type HandlerMap<C> = Arc<Mutex<HashMap<String, Arc<dyn MessageHandler<C>>>>>;

pub struct Node<C> {
    pub node_id: Option<String>,
    pub next_msg_id: u32,
    pub node_ids: Vec<Option<String>>,
    handlers: HandlerMap<C>,
    pub ctx: C,
}

impl<C> Node<C> {
    pub fn new(ctx: C) -> Self {
        let mut node = Self {
            node_id: None,
            next_msg_id: 0,
            node_ids: vec![],
            handlers: Arc::new(Mutex::new(HashMap::new())),
            ctx,
        };

        node.register_handler("init", InitHandler {});
        node.register_handler("echo", EchoHandler {});

        node
    }

    pub fn register_handler<T: MessageHandler<C> + 'static>(&mut self, msg_type: &str, handler: T) {
        let mut handlers = self.handlers.lock().unwrap();
        handlers.insert(msg_type.to_string(), Arc::new(handler));
    }

    pub fn log(&self, message: impl AsRef<str>) {
        let mut stderr = io::stderr().lock();
        writeln!(stderr, "{}", message.as_ref()).unwrap();
        stderr.flush().unwrap();
    }

    pub fn send(&self, dest: String, body: MessageBody) {
        let mut stdout = io::stdout().lock();
        let message =
            json!({
            "src": self.node_id,
            "dest": dest,
            "body": body,
        });
        let json_string = serde_json::to_string(&message).unwrap();
        writeln!(stdout, "{}", json_string).unwrap();
        stdout.flush().unwrap();
    }

    fn handle_message(&mut self, msg: Message) {
        let msg_type = msg.msg_type().to_string();
        let handler_opt = {
            let handlers = self.handlers.lock().unwrap();
            handlers.get(&msg_type).cloned()
        };
        if let Some(handler) = handler_opt {
            if let Err(e) = handler.handle(self, msg) {
                self.log(format!("Handler error: {}", e));
            }
        } else {
            let ignored = ["broadcast_ok", "read_ok", "init_ok", "echo_ok", "topology_ok"];
            if !ignored.contains(&msg_type.as_str()) {
                self.log(format!("No handler for message type: {}", msg_type));
            }
        }
    }

    pub fn main_loop(&mut self) {
        let stdin = io::stdin();
        std::panic::set_hook(
            Box::new(|info| {
                eprintln!("PANIC: {:?}", info);
            })
        );
        for line in stdin.lock().lines() {
            match line {
                Ok(input) => {
                    match serde_json::from_str::<Message>(&input) {
                        Ok(msg) => self.handle_message(msg),
                        Err(e) => self.log(format!("Error parsing message: {}", e)),
                    }
                }
                Err(e) => self.log(format!("Error reading input: {}", e)),
            }
        }
    }
}
