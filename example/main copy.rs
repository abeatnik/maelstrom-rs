use maelstrom_rs::node::Node;
use maelstrom_rs::message::Request;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    let mut node: Option<Node> = None;

    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            let request: Request = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(_) => continue,
            };

            if request.body.r#type == "init" {
                node = Some(Node::new(
                    request.src.clone(),
                    vec![], // No peers yet
                ));
            }

            if let Some(ref n) = node {
                if let Some(response) = n.handle_message(request) {
                    let json_response = serde_json::to_string(&response).unwrap();
                    writeln!(stdout, "{}", json_response).unwrap();
                    stdout.flush().unwrap();
                }
            }
        }
    }
}
