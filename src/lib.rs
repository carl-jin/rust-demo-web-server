mod worker;
use crate::worker::*;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

//  todo 阅读逻辑，达到完全搞懂和重新的能力
//  todo 添加优雅关闭
//  todo 思路如下
//  ThreadPool 
//      workers 用来储存 worker
//          thread  用于储存 线程
//      sender  用来储存 sender

//  threadpool 生成 workers 时同时创建一个 channel， 
//  将 sender 保存在 threadpool 上， 而 receiver 通过 Arc 智能指针 和 Mutex 来传递给 worker 这中的 thread
//  通过 threadpool 上实现的 execute 方法，调用 self.sender 将信息发送给 worker 下面的 reciver 达到执行闭包的功能
//  

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// 创建线程
    ///
    /// # Panics
    /// `new` 函数在 size 为 0 时会 panic
    pub fn new(size: u8) -> ThreadPool {
        assert!(size > 0);

        let mut workers: Vec<Worker> = Vec::with_capacity(size.into());
        let (sender, receiver) = mpsc::channel::<Job>();

        let receiver = Arc::new(Mutex::new(receiver));

        for i in 0..size {
            workers.push(Worker::new(i.into(), Arc::clone(&receiver)));
        }

        ThreadPool {
            workers: workers,
            sender: sender,
        }
    }

    /// 执行 闭包
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).unwrap()
    }
}

impl Drop for ThreadPool{
    fn drop(&mut self){
        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
