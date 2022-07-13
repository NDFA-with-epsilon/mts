// use std::{fmt, thread};
// #[derive(Debug, Clone)]
// pub struct PoolCreationError;
// impl fmt::Display for PoolCreationError
// {
//     fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
//     {
//         write!(f, "Cannot create empty thread pool")
//     }
// }

use std::thread;
use std::sync::{Arc, Mutex, mpsc};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool
{
    worker_threads: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool
{
    pub fn new(size: usize) -> ThreadPool
    {
        let max_size = 10;
        assert!(size <= max_size);

        let (sender, receiver) = mpsc::channel();
        let receiver_wrapped = Arc::new(Mutex::new(receiver));

        let mut worker_threads = Vec::with_capacity(size);

        for id in 0..size
        {
            worker_threads.push(Worker::new(id, Arc::clone(&receiver_wrapped)));
        }

        ThreadPool { worker_threads, sender }
    }

    pub fn execute<F>(&self, f: F) 
    where 
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

struct Worker
{
    id: usize, 
    thread: thread::JoinHandle<()>,
}

impl Worker
{
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self
    {
        let thread = thread::spawn(move|| loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker thread {} assigned for job.", id);
            job();
        });
        Worker{id, thread}
    }
}