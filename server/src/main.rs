use cl::prelude::*;
use ws::{ Handler, Handshake, Sender, Result, Message };
use std::{
    thread,
    sync::mpsc,
};

struct Server {
    out: Sender,
}

impl Handler for Server {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("on_open");

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
//        println!("Got: {}", msg);

        Ok(())
    }
}

fn main() {
    println!("Server started");

    let mut socket = ws::WebSocket::new(|out| {
        Server { out }
    }).unwrap();

    let broadcaster = socket.broadcaster();

    let sock_handler = thread::spawn(|| {
        socket.listen("127.0.0.1:1337").unwrap();
    });

    // Send messages in a loop
    let mut count: u64 = 1;
    loop {
        let msg = FooMessage::new(Foo::Status {
            seq: count,
            time: Utc::now(),
        });
        broadcaster.broadcast(ron::ser::to_string(&msg).unwrap());
        println!("tick {:?}", msg);
        count += 1;
        cl::sleep!(5 ms);
    }

    sock_handler.join();

    println!("Server finished");
}