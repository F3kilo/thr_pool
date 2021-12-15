use std::sync::mpsc::{Receiver, RecvError, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread::JoinHandle;
use std::{mem, thread};

type Task = Box<dyn FnOnce() + Send + 'static>;

#[derive(Debug, Clone)]
pub struct ThreadPool {
    handles: Arc<Vec<JoinHandle<()>>>,
    sender: Option<Sender<Task>>,
}

impl ThreadPool {
    pub fn new(threads_count: usize) -> Option<Self> {
        if threads_count == 0 {
            return None;
        }

        let (sender, rx) = mpsc::channel::<Task>();
        let rx = Arc::new(Mutex::new(rx));

        let mut handles = Vec::with_capacity(threads_count);
        for _ in 0..threads_count {
            let rx = rx.clone();
            let jh = thread::spawn(move || {
                while let Ok(t) = get_task(&rx) {
                    t()
                }
            });
            handles.push(jh);
        }

        Some(Self {
            handles: Arc::new(handles),
            sender: Some(sender),
        })
    }

    pub fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let task = Box::new(f);
        let _ = self.sender.as_ref().unwrap().send(task);
    }
}

fn get_task(recv: &Mutex<Receiver<Task>>) -> Result<Task, RecvError> {
    recv.lock().unwrap().recv()
}

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
