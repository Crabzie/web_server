//! # ThreadPool
//!
//! a collection of utilities to make creation of threadpool easier by spawning dead worker threads until they are used.

use core::panic;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// Pool creation errors enum
#[derive(Debug)]
pub enum PoolCreationError {
    ZeroSizedPool,
}

/// Struct that hold id and handler of a worker thread.
struct Worker {
    id: usize,
    handler: thread::JoinHandle<()>,
}

impl Worker {
    /// Create a new worker thread that is dead until used.
    /// Takes `id` the identifier of the worker thread, `receiver` a channel receiving end to receive jobs from.
    ///
    /// # Examples
    ///
    /// ```
    /// let size = 4;
    /// let mut workers = Vec::with_capacity(size);
    ///
    /// for id in 0..size {
    ///    workers.push(Worker::new(id));
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// Function will panic if system doesn't have enough resources to spawn new threads.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let builder = thread::Builder::new().name(id.to_string());
        let handler = builder
            .spawn(move || loop {
                let message = receiver.lock().unwrap().recv();

                match message {
                    Ok(job) => {
                        println!("Thread Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Thread Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            })
            .unwrap_or_else(|error| panic!("Error at creating worker thread: {error:?}"));
        Worker { id, handler }
    }
}

/// Struct that groups a vector of worker threads & a channel sender side to send jobs to workers.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

/// type aliase for worker jobs i.e `closures`
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Builds a new ThreadPool, The `size` is the number of worker threads in the pool.
    /// Communication with worker threads is done via thread channels respecting Mutual exclusion.
    ///
    /// # Examples
    ///
    /// ```
    /// let pool = ThreadPool::build(4).unwrap_or_else(
    ///    |error| panic!("Problem creating thread pool: {error:?}")
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// Function can return this set of errors:
    ///
    /// - NullSize: Building a thread pool with zero size.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError::ZeroSizedPool);
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Execute a job on a free worker thread, the job is represented by a `closure`.
    ///
    /// # Panic
    ///
    /// An unsuccessful send would be one where the corresponding receiver has already been deallocated.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .unwrap()
            .send(job)
            .unwrap_or_else(|error| panic!("Channel send panic: {error:?}"));
    }
}

impl Drop for ThreadPool {
    /// Drop mechanism for closing threads. The procedure is:
    ///
    /// - Cut the channel connection to cut the recv() listen loop.
    /// - Call join on thread handlers to wait a global closing of all threads.
    ///
    /// # Panic:
    ///
    /// If a thread attempts to join itself or otherwise may create a deadlock with joining threads.
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);

            worker
                .handler
                .join()
                .unwrap_or_else(|error| panic!("Thread join panic: {error:?}"));
        }
    }
}
#[cfg(test)]
mod thread_pool {
    use super::*;
    use core::panic;
    #[should_panic]
    #[test]
    /// Zero sized thread pool test
    fn empty_size_pool() {
        ThreadPool::build(0)
            .unwrap_or_else(|error| panic!("Problem creating thread pool: {error:?}"));
    }
}
