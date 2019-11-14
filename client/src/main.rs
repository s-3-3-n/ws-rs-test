use cl::prelude::*;
use ws::{ Handler, Handshake, Sender, Result, Message };
use std::{
    rc::Rc,
    cell::RefCell,
};
use std::borrow::BorrowMut;

struct Stats {
    count: u64,
    highest: u64,
    average: f64,
    last_seq: u64,
    missed: u64,
}

impl Stats {
    fn update(&mut self, seq: u64, data: u64) {
        self.average = (self.average * (self.count as f64 / (self.count as f64 + 1.0))) + (data as f64 * (1.0 / (self.count as f64 + 1.0)));
        self.count += 1;
        if data > self.highest {
            self.highest = data;
        }

        if self.last_seq != 0 {
            if seq > self.last_seq + 1 {
                self.missed += 1;
            }
        }
        self.last_seq = seq;
    }
}

// TODO: This is dumb, move stats output to somewhere more sane
impl std::fmt::Debug for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Stats:")?;
        writeln!(f, "Count: {}", self.count)?;
        writeln!(f, "Seq: {}", self.last_seq)?;
        writeln!(f, "Missed: {}", self.missed)?;
        writeln!(f, "Highest: {} ms", self.highest as f64  / 1_000_000.0)?;
        writeln!(f, "Average: {} ms", self.average / 1_000_000.0)
    }
}

struct Client {
    out: Sender,
    stats: RefCell<Stats>,
}

impl Handler for Client {
    fn on_open(&mut self, shake: Handshake) -> Result<()> {
        println!("on_open");

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        print!("{}[2J", 27 as char); // Clears the terminal

        // We expect that all messages are of type FooMessage
        let msg: FooMessage = ron::de::from_bytes::<FooMessage>(msg.into_data().as_ref()).unwrap();
        let mut stats = self.stats.borrow_mut();

        match msg.msg {
            // Currently the only message available
            // Meant to track how long it takes from the message's creation on the server
            Foo::Status{ seq, time } => {
                let diff = Utc::now() - time;
                stats.update(seq,diff.num_nanoseconds().unwrap() as u64);
            },
            _ => {}
        }
        println!("Got: {:?}", msg);
        println!("{:?}", stats);

        self.out.send("pong")
    }
}

fn main() {
    println!("Client started");

    ws::connect("ws://127.0.0.1:1337", |out| {
        Client {
            out,
            stats: RefCell::new(Stats {
                count: 0,
                highest: 0,
                average: 0.0,
                last_seq: 0,
                missed: 0,
            })
        }
    }).unwrap();

    println!("Client finished");
}