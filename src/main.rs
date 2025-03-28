use maelstrom_rs::echo::EchoServer;

fn main() {
    let mut server = EchoServer::new();
    server.main();
}
