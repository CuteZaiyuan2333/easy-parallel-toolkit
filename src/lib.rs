use std;
use std::sync::{
    Arc,
    atomic::*
};

pub struct RuntimeMachine<T1: Send + 'static>{
    total: i32,
    done: Arc<AtomicI32>,
    max_threads: i32,
    handles: Vec<std::thread::JoinHandle<T1>>,
    tasks: Vec<Box<dyn Runnable<T1> + Send + 'static>>,
    pub result: Vec<T1>,
}impl<T1: Send + 'static> RuntimeMachine<T1> {
    pub fn init()->Self{
        Self{
            total: 0,
            // processing: 0,
            done: Arc::new(AtomicI32::new(0)),
            max_threads: 4,
            handles: Vec::new(),
            tasks: Vec::new(),
            result: Vec::new(),
        }
    }
    pub fn static_push(self: &mut Self, runnable: impl Runnable<T1> + Send + 'static){
        self.tasks.push(Box::new(runnable));
        self.total += 1;
    }
    pub fn get_max_threads(self: &Self) -> i32{
        return self.max_threads;
    }
    pub fn set_max_threads(self: &mut Self, input: i32){
        self.max_threads = input;
    }
    pub fn compute(self: &mut Self, threads: i32){
        for _ in 0..threads{
            if self.tasks.len() > 0{
                let mut task = self.tasks.pop().unwrap();
                let count = Arc::clone(&self.done);
                let handle = std::thread::spawn(move ||{
                    let result = task.enter();
                    count.fetch_add(1, Ordering::SeqCst);
                    result
                });
                self.handles.push(handle);
            }
        }
    }
    pub fn inquiry(self: &Self) -> State<i32>{
        let count = self.done.load(Ordering::SeqCst);
        match (self.total, count) {
            (0, 0) =>{
                return State::Empty;
            }
            (_, _) =>{
                return State::Number(self.total, count)
            }
        }
    }
    pub fn obtain(self: &mut Self) {
        while !self.tasks.is_empty() || !self.handles.is_empty() {
            while self.handles.len() < self.max_threads as usize && !self.tasks.is_empty() {
                let mut task = self.tasks.pop().unwrap();
                let count = Arc::clone(&self.done);
                let handle = std::thread::spawn(move || {
                    let result = task.enter();
                    count.fetch_add(1, Ordering::SeqCst);
                    result
                });
                self.handles.push(handle);
            }
            for handle in self.handles.drain(..) {
                self.total -= 1;
                if let Ok(res) = handle.join() {
                    self.result.push(res);
                    self.done.fetch_add(-1, Ordering::SeqCst);
                }
            }
        }
    }
}

pub trait Runnable<T1>{
    fn enter(self: &mut Self) -> T1;
}

pub enum State<T>{
    Empty,
    Number(T, T),
    Error,
}