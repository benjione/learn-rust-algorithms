use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};


pub struct ThreadPool {
    max_threads: usize,
    thread_handle_queue: Vec<thread::JoinHandle<()>>,
}

fn exec<F>(fun: F) -> () {


}

impl ThreadPool {
    pub fn new(max_threads: usize) -> ThreadPool {
        let mut vec = vec![];
        for i in 0..max_threads {
            let (tx, rx): (Sender<exec>, Receiver<i32>) = mpsc::channel();
            vec.push(thread::spawn(move ||{
                for exec in rx {
                    //exec();
                }
            }))
        }
        ThreadPool{
            max_threads: max_threads,
            thread_handle_queue: vec,
        }
    }

    pub fn execute() -> () {

    }
}
