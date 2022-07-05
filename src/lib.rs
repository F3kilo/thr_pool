pub mod actor;

use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{mem, thread};

type Task = Box<dyn FnOnce() + Send + 'static>;

/// Basic thread pool
#[derive(Debug, Clone)]
pub struct ThreadPool {
    handles: Arc<Vec<JoinHandle<()>>>,
    sender: Option<Sender<Task>>,
}

impl ThreadPool {
    /// Create thread pool with `threads_count` system threads
    pub fn new(threads_count: usize) -> Option<Self> {
        if threads_count == 0 {
            return None;
        }

        // channel to communicate with threads
        let (sender, rx) = mpsc::channel::<Task>();

        // Receiver<T>: !Sync, mutex required
        let rx = Arc::new(Mutex::new(rx));

        // Run system threads
        let handles = (0..threads_count)
            .map(|_| {
                let rx = rx.clone();
                thread::spawn(move || {
                    while let Ok(t) = get_task(&rx) {
                        t()
                    }
                })
            })
            .collect();

        Some(Self {
            handles: Arc::new(handles),
            sender: Some(sender),
        })
    }

    /// Spawn new task for one of threads
    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // send boxed task to threads
        let task = Box::new(f);
        let _ = self.sender.as_ref().unwrap().send(task);
    }
}

fn get_task(recv: &Mutex<Receiver<Task>>) -> Result<Task, RecvError> {
    recv.lock().unwrap().recv()
}

/// Waits, when all threads finish their jobs
impl Drop for ThreadPool {
    fn drop(&mut self) {
        let arc = mem::replace(&mut self.handles, Arc::new(Default::default()));
        if let Ok(handles) = Arc::try_unwrap(arc) {
            mem::drop(self.sender.take());
            for jh in handles {
                jh.join().unwrap();
            }
        }
    }
}
