use std::{
    sync::{
        Arc,
        atomic::AtomicI32,
        mpsc::{
            Receiver,
            Sender,
            channel
        }
    }, thread::{
        self,
        JoinHandle
    }
};

pub struct PoolThread<T1: Runnable<T2> + Send + 'static, T2: Send + 'static>{
    at_counter: Arc<AtomicI32>,
    tx: Option<Sender<T1>>,
    rx: Receiver<T2>,
    handle: Option<JoinHandle<()>>,
}impl<T1: Runnable<T2> + Send + 'static, T2: Send + 'static> PoolThread<T1, T2>{
    pub fn init() -> Self{
        let at_counter = Arc::new(AtomicI32::new(0));
        let counter_move = Arc::clone(&at_counter);
        let (tx, rx_move):(Sender<T1>, Receiver<T1>) = channel::<T1>();
        let (tx_move, rx):(Sender<T2>, Receiver<T2>) = channel::<T2>();
        let handle = thread::spawn(move || {
            for task in rx_move{
                let result = task.run();
                counter_move.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                tx_move.send(result).unwrap();
            }
        });
        Self{
            at_counter,
            tx: Some(tx),
            rx,
            handle: Some(handle),
        }
    }
    pub fn push(self: &mut Self, task: T1){
        self.at_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if let Some(ref tx) = self.tx{
            tx.send(task).unwrap();
        }else {
            self.at_counter.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    pub fn pop(self: &mut Self) -> Vec<T2>{
        // let result = self.rx.try_recv().ok();
        // return result;
        let mut result: Vec<T2> = Vec::new();
        while let Ok(msg) = self.rx.try_recv(){
            result.push(msg)
        }
        return result;
    }
    pub fn inquiry(self: &Self) -> i32{
        return self.at_counter.load(std::sync::atomic::Ordering::SeqCst)
    }
}

impl<T1: Runnable<T2> + Send + 'static, T2: Send + 'static> Drop for PoolThread<T1, T2> {
    fn drop(&mut self) {
        let _ = self.tx.take(); 
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

pub trait Runnable<T>{
    fn run(self: Self) -> T;
}