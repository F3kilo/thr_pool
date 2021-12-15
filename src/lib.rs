#[derive(Debug, Clone)]
pub struct ThreadPool;

impl ThreadPool {
    pub fn new(_threads: u32) -> Option<Self> {
        todo!()
    }

    pub fn spawn<F>(&self, _f: F)
    where
        F: FnOnce() + 'static,
    {
        todo!()
    }
}