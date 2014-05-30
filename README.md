# rust-irc
rust-irc is an IRC client/server Rust crate.

## Example

```rust
extern crate irc;

use irc::server;

fn main() {
  let server = IRCServer::new("127.0.0.1", 6697)

  // Define closure that prints nicknames that join the server
  let print_join = |nickname| { println!("Joined: ", nickname) }

  // Give the server our closure so it will call it on join
  server.on_join(print_join)

  // Start the server
  server.start()
}
```
