use std::time::Duration;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
};
type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, reciver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                //  todo
                //  recv 方法会修改 reviecer 的值吗，不然为什么要放到互斥锁里面
                let job = reciver.lock().unwrap().recv().unwrap();
                println!("Worker {} got a job; executing.", id);

                // println!("before sleep {}",id);
                // thread::sleep(Duration::from_secs(120));
                // println!("after sleep {}",id);

                //  todo 理解下这个 job sleep 后为什么会阻塞其他线程的运行
                //  https://course.rs/advance/concurrency-with-threads/sync1.html#%E8%AF%BB%E5%86%99%E9%94%81rwlock
                //  看看上面这个读写锁
                //  测试下 https://kaisery.github.io/trpl-zh-cn/ch20-03-graceful-shutdown-and-cleanup.html
                //  最终的代码是否也存在这个问题
                //  为什么要加个锁
                // println!("before run bibao");
                job();
                // println!("after run bibao");
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
