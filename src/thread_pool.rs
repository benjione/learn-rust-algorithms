use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Arc;
use std::sync::Mutex;


pub struct ThreadPool {
    max_threads: usize,
    thread_handle_queue: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

trait FnBox {
    fn call_box(self: Box<Self>) -> WorkerMsg;
}

enum WorkerMsg {
    keep_running,
    stop,
}

impl<F: FnOnce() -> WorkerMsg> FnBox for F {
    fn call_box(self: Box<F>) -> WorkerMsg {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

impl Worker {
    pub fn new(id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                // println!("Thread nr. {} received func!", id);
                match job.call_box() {
                    WorkerMsg::keep_running => {},
                    WorkerMsg::stop => {
                        return;
                    },
                };
            }
        });
        Worker{
            id,
            thread,
        }
    }

    pub fn stop(self) -> () {
        self.thread.join();
    }
}


impl ThreadPool {
    pub fn new(max_threads: usize) -> ThreadPool {
        let mut vec = vec![];
        let (tx, rx): (Sender<Job>, Receiver<Job>) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        for i in 0..max_threads {
            vec.push(Worker::new(i, Arc::clone(&rx)))
        }
        ThreadPool{
            max_threads: max_threads,
            thread_handle_queue: vec,
            sender: tx,
        }
    }

    pub fn kill_thradpool(self) -> () {
        for a in 0..self.max_threads {
            self.sender.send(Box::new(move || {
                return WorkerMsg::stop;
            })).unwrap();
        }
        for worker in self.thread_handle_queue {
            worker.stop();
        }
    }

    pub fn execute<F>(&self, fun: F) -> ()
        where F: FnOnce() + Send + 'static
    {
        let fun = || {fun(); return WorkerMsg::keep_running;};
        let job = Box::new(fun);
        self.sender.send(job).unwrap();
    }

    pub fn exec_with_return_value<F,T>(&self, fun: F) -> T
        where F: FnOnce() -> T + Send + 'static,
        T: Send + 'static
    {
        let (tx, rx) = mpsc::channel();
        self.execute(move || tx.send(fun()).unwrap());
        let ret = rx.recv().unwrap();
        return ret;
    }

}



#[cfg(test)]
mod tests {
    use super::ThreadPool;

    #[test]
    fn test_adding_once() {
        let threads = ThreadPool::new(4);
        let a = 5;
        let (tx, rx) = super::mpsc::channel();
        threads.execute(move || {tx.send(a+1).unwrap();});
        let res = rx.recv().unwrap();
        threads.kill_thradpool();
        assert_eq!(res, 6);
    }

    #[test]
    fn test_adding_loop() {
        let threads = ThreadPool::new(4);
        for a in 1..1000 {
            let (tx, rx) = super::mpsc::channel();
            threads.execute(move || {tx.send(a+1).unwrap();});
            let res = rx.recv().unwrap();
            assert_eq!(res, a+1);
        }
        threads.kill_thradpool();
    }

    #[test]
    fn test_adding_with_ret() {
        let threads = ThreadPool::new(4);
        let a = 5;
        let res = threads.exec_with_return_value(move || a+1);
        threads.kill_thradpool();
        assert_eq!(res, 6);
    }

}
