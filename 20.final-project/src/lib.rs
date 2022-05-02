use std::{
    error::Error,
    fmt::Display,
    sync::{mpsc, Arc, Mutex},
    thread::{self, Thread},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// To relieve the serialized problem in a single-threade server,
/// utilized the thread pool.
/// Create a thread pool having a number of threads, and then
/// use them as workers.
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the numer of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            Err(PoolCreationError(size))
        } else {
            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);
            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }
            Ok(ThreadPool { workers, sender })
        }
    }

    /// Send messages to the channel,
    /// then workers will receive and handle the message.
    /// Message will contain a closure to execute
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .send(Message::NewJob(job))
            .expect("Unable to send NewJob message to the workers");
    }
}

/// To gracefully shutdown the threadpool,
/// we should wait for the workes who are still handling the message.
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");
        // send termination messages for the number of workers
        for _ in &self.workers {
            self.sender
                .send(Message::Terminate)
                .expect("Unable to send Terminate message to the workers");
        }
        println!("Shutting down all workers.");
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread
                    .join()
                    .expect(format!("Unable to wait for the worker id {}", worker.id).as_str());
            }
        }
    }
}

// A worker thread who is polling the message from the main thread.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    /// A newly created worker will loop forever,
    /// until it gets `Terminate` message.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("Unable to lock the mutex. There might be panic'ed thread.")
                .recv()
                .expect("Unable to receive messages from the main thread. The channel might have been gone.");
            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job: executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// A cutom Error
#[derive(Debug)]
pub struct PoolCreationError(usize);

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unable to create thread pool containing {} threads",
            self.0
        )
    }
}

enum Message {
    NewJob(Job),
    Terminate,
}
