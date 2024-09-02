use std::io::stdin;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use thr_pool::actor::{Actor, System};

fn main() {
    let mut system = System::default();

    let pong = PongActor;
    let pong_tx = system.run(pong);

    let ping = PingActor::new(pong_tx);
    let ping_tx = system.run(ping);

    let input = InputActor::new(ping_tx);
    let input_tx = system.run(input);

    input_tx.send(()).unwrap();
}

// **** INPUT ****

struct InputActor(Sender<PingMessage>);

impl InputActor {
    pub fn new(ping_tx: Sender<PingMessage>) -> Self {
        Self(ping_tx)
    }
}

type InputMessage = ();

impl Actor for InputActor {
    type Message = InputMessage;

    fn process_message(self, _: Self::Message) -> Option<Self> {
        loop {
            println!();
            println!("Enter message for ping-pong:");
            let mut msg = String::new();
            stdin().read_line(&mut msg).ok()?;
            let msg = msg.trim().to_string();

            if msg == "exit" {
                return None;
            }

            self.0.send(PingMessage::new(msg)).ok()?;
            thread::sleep(Duration::from_secs(1));
        }
    }
}

// **** PING ****

struct PingActor(Sender<PongMessage>);

impl PingActor {
    pub fn new(pong_tx: Sender<PongMessage>) -> Self {
        Self(pong_tx)
    }
}

struct PingMessage(String);

impl PingMessage {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Actor for PingActor {
    type Message = PingMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let str = msg.into_string();
        println!("ping with message: {}", str);
        self.0.send(PongMessage::new(str)).ok()?;
        Some(self)
    }
}

// **** PONG ****

struct PongActor;

struct PongMessage(String);

impl PongMessage {
    pub fn new(text: String) -> Self {
        Self(text)
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Actor for PongActor {
    type Message = PongMessage;

    fn process_message(self, msg: Self::Message) -> Option<Self> {
        let str = msg.into_string();
        println!("pong with message: {}", str);
        Some(self)
    }
}
