mod irc_server;

fn main() {
  let server = irc_server::IRCServer::new(String::from_str("127.0.0.1"), 8181u16);
  server.start();
}
