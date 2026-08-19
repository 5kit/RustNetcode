use std::{
    format, println,
    sync::{Arc, Mutex, mpsc},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    /// The size is the number of threads in the pool.
    pub fn new(size: usize) -> ThreadPool {
        ThreadPool::build(size).expect("ThreadPool size must be greater than zero")
    }

    /// The `build()` function will return a PoolCreationError if size is zero.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let (sender, receiver) = mpsc::channel();
            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool {
                size,
                workers,
                sender: Some(sender),
            })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F) -> Result<(), PoolExecutionError>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        let sender = match &self.sender {
            Some(sender) => sender,
            None => return Err(PoolExecutionError),
        };

        sender.send(job).map_err(|_| PoolExecutionError)
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in self.workers.drain(..) {
            worker.thread.join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = thread::Builder::new().name(format!("Worker-{id}").into());
        let thread = builder
            .spawn(move || {
                loop {
                    let message = receiver.lock().unwrap().recv();

                    match message {
                        Ok(job) => {
                            println!("Worker {id} got a job; executing.");
                            job();
                        }
                        Err(_) => {
                            println!("Worker{id} is shutting down.");
                            break;
                        }
                    }
                }
            })
            .unwrap();
        Worker { id, thread }
    }
}

// -------------------------------------------------------
// Error Handling Types
// -------------------------------------------------------

#[derive(Debug)]
pub struct PoolCreationError;
#[derive(Debug)]
pub struct PoolExecutionError;
#[derive(Debug)]
pub struct NoRequestError;

impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        ServerError::Io(error)
    }
}

impl From<PoolCreationError> for ServerError {
    fn from(error: PoolCreationError) -> Self {
        ServerError::PoolCreation(error)
    }
}

impl From<PoolExecutionError> for ServerError {
    fn from(error: PoolExecutionError) -> Self {
        ServerError::PoolExecution(error)
    }
}

impl From<NoRequestError> for ServerError {
    fn from(error: NoRequestError) -> Self {
        ServerError::NoRequest(error)
    }
}

pub struct ThreadPool {
    pub size: usize,
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

#[derive(Debug)]
pub enum ServerError {
    Io(std::io::Error),
    PoolCreation(PoolCreationError),
    PoolExecution(PoolExecutionError),
    NoRequest(NoRequestError),
}
