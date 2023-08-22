use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub trait Actor: Sized + Send + 'static {
    type Message: Send + 'static;

    fn process_message(self, msg: Self::Message) -> Option<Self>;

    fn name() -> &'static str {
        std::any::type_name::<Self>()
    }
}

/// Basic actor framework
#[derive(Debug, Default)]
pub struct System {
    handles: Vec<JoinHandle<()>>,
}

impl System {
    /// Run `actor`. It'll wait for messages and process them.
    /// Method returns channel to communicate with `actor`.
    pub fn run<A: Actor>(&mut self, actor: A) -> Sender<A::Message> {
        let (tx, rx) = mpsc::channel();
        let jh = thread::spawn(move || {
            println!("actor {} started", A::name());
            let mut actor = actor;
            while let Ok(msg) = rx.recv() {
                actor = match actor.process_message(msg) {
                    Some(a) => a,
                    None => break,
                }
            }
            println!("actor {} finished", A::name());
        });
        self.handles.push(jh);

        tx
    }
}

impl Drop for System {
    /// Waits when all actors finish their work.
    fn drop(&mut self) {
        let handles = std::mem::take(&mut self.handles);
        for jh in handles {
            jh.join().unwrap();
        }
    }
}
