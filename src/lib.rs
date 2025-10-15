use std::{
    sync::{Arc, Mutex, mpsc},
    thread,
};

/// Data Structure to identify and store the threads for our server
pub struct Worker {
    /// Unique identifier for each thread
    id: usize,
    /// Wrapping each thread inside an option since threads are later terminated when shutting down
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Create a new thread and assign it to Worker stuct
        let thread = thread::spawn(move || {
            loop {
                // For the cuurent thread - acquires a mutex, blocking the current thread until the job is complete.
                let message = receiver.lock().unwrap().recv();

                // Match whther the result against its variants to ensure that the thread is valid.
                match message {
                    // If the thread is valid
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        // Perform the job/action
                        job();
                    }

                    // If the session is terminated - the thread is then considered an error
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    /// Vector of all the worker threads available for execution
    worker_instances: Vec<Worker>,

    ///
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        // The `new` function will panic if the size is zero.
        assert!(size > 0);

        // Create a MPSC channel for threads to communicate amongst each other
        let (sender, receiver) = mpsc::channel();
        // Individual threads
        let receiver = Arc::new(Mutex::new(receiver));

        // vector of defined size to store threads
        let mut workers = Vec::with_capacity(size);

        for i in 0..size {
            // Push the threads into Worker struct
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }

        // Initialize the thread-pool
        ThreadPool {
            worker_instances: workers,
            sender: Some(sender),
        }
    }

    /*
        Takes the closure it’s given and gives it to an idle thread in the pool to run.

        We need an interface similar to thread::spawn implementation:
            pub fn spawn<F, T>(f: F) -> JoinHandle<T>
            where
                F: FnOnce() -> T,
                F: Send + 'static,
                T: Send + 'static,
            {
                Builder::new().spawn(f).expect("failed to spawn thread")
            }

        The F type parameter is the one we’re concerned with here; the T type parameter is related to the return value, and we’re not concerned with that for now.

        We can see that spawn uses FnOnce as the trait bound on F. This is probably what we want as well, because we’ll eventually pass the argument we get in execute to spawn. We can be further confident that FnOnce is the trait we want to use because the thread for running a request will only execute that request’s closure one time, which matches the Once in FnOnce.

        We need Send to transfer the closure from one thread to another and 'static because we don’t know how long the thread will take to execute.
    */

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

// Graceful shutdown handling for threads
impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Consumes the channel - takes the value out of the option, leaving a None in its place
        drop(self.sender.take());

        for worker in &mut self.worker_instances {
            println!("Shutting down worker {}", worker.id);

            // Consumes each thread inside the channel
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}
