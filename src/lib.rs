pub mod poolthread;
use crate::poolthread::{
    PoolThread,
    Runnable,
};

pub struct Warp<T1: Runnable<T2> + Send + 'static, T2: Send + 'static>{
    threads_number: i32,
    threads: Vec<PoolThread<T1, T2>>,
    payloads: Vec<T1>,
}impl<T1: Runnable<T2> + Send + 'static, T2: Send + 'static> Warp<T1, T2>{
    pub fn init() -> Self{
        Self{
            threads_number: 4,
            threads: Vec::new(),
            payloads: Vec::new(),
        }
    }

    pub fn set_thread_number(self: &mut Self, input: u32){
        self.threads_number = input as i32;
    }
    
    pub fn startup(self: &mut Self){
        for _ in 0..self.threads_number{
            self.threads.push(PoolThread::init());
        }
    }
    
    pub fn push(self: &mut Self, task: T1){
        self.payloads.push(task);
    }

    pub fn compute(self: &mut Self){
        if self.threads.is_empty(){
            return;
        }

        let mut loads: Vec<i32> = self.threads
            .iter()
            .map(|thread| thread.inquiry())
            .collect();

        while let Some(task) = self.payloads.pop(){
            // 每次都选择当前负载最低的线程，并在本地同步增加负载计数。
            // 这样负载越低的线程优先级越高，也会在一轮 compute 中获得更多任务。
            let target = (0..self.threads.len())
                .min_by_key(|&index| loads[index])
                .unwrap();
            self.threads[target].push(task);
            loads[target] += 1;
        }
    }

    pub fn pop(self: &mut Self) -> Vec<T2>{
        let mut results: Vec<T2> = Vec::new();
        let mut rewind: Vec<PoolThread<T1, T2>> = Vec::new();
        for _ in 0..self.threads_number{
            let mut temp = self.threads.pop().unwrap();
            let mut stack = temp.pop(); 
            rewind.push(temp);
            for item in stack.drain(..){
                results.push(item);
            }
        }
        for thread in rewind.drain(..){
            self.threads.push(thread);
        }
        return results;
    }
}
