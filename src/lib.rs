use std;

pub struct RuntimeMachine<T1: Send + 'static>{
    remain: i32,
    processing: i32,
    max_threads: i32,
    handles: Vec<std::thread::JoinHandle<T1>>,
    tasks: Vec<Box<dyn Runnable<T1> + Send + 'static>>,
    pub result: Vec<T1>,
}impl<T1: Send + 'static> RuntimeMachine<T1> {
    pub fn init()->Self{
        Self{
            remain: 0,
            processing: 0,
            max_threads: 4,
            handles: Vec::new(),
            tasks: Vec::new(),
            result: Vec::new(),
        }
    }
}

impl<T1: Send + 'static> RuntimeMachine<T1>{
    pub fn static_push(self: &mut Self, runnable: impl Runnable<T1> + Send + 'static){
        self.tasks.push(Box::new(runnable));
        self.remain += 1;
    }
    pub fn compute(self: &mut Self, threads: i32){
        for _ in 0..threads{
            if self.tasks.len() > 0{
                let mut task = self.tasks.pop().unwrap();
                let handle = std::thread::spawn(move ||{
                    task.enter()
                });
                self.handles.push(handle);
                self.remain -= 1;
                self.processing += 1;
            }
        }
    }
    pub fn inquiry(self: &mut Self) -> State<i32>{
        match (self.remain, self.processing) {
            (0, 0) =>{
                return State::Empty;
            }
            (_, _) =>{
                return State::Number(self.remain, self.processing)
            }
        }
    }
    pub fn obtain(self: &mut Self) {
        while !self.tasks.is_empty() || !self.handles.is_empty() {
            while self.handles.len() < self.max_threads as usize && !self.tasks.is_empty() {
                let mut task = self.tasks.pop().unwrap();
                let handle = std::thread::spawn(move || task.enter());
                self.handles.push(handle);
                self.remain -= 1;
                self.processing += 1;
            }
            for handle in self.handles.drain(..) {
                if let Ok(res) = handle.join() {
                    self.result.push(res);
                    self.processing -= 1;
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