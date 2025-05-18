use maelstrom_rs::node::Node;
use maelstrom_rs::broadcast::Broadcast;

fn main() {
    let mut broadcast = Broadcast::new();
    broadcast.node.main_loop();
}
