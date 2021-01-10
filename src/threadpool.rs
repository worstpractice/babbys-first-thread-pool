use crate::{
  types::Job,
  worker::Worker,
};
use std::sync::{
  mpsc,
  Arc,
  Mutex,
};

#[allow(unused)]
pub struct ThreadPool {
  workers: Vec<Worker>,
  /// The sole transmitter held by the `ThreadPool`.
  tx: mpsc::Sender<Job>,
}

impl ThreadPool {
  pub fn new(pool_size: usize) -> ThreadPool {
    // Bail if the pool size is zero (the only non-positive value a `usize` can represent)
    assert!(pool_size != 0);

    let (tx, rx) = mpsc::channel();

    // rx is now `Sync`
    let rx = Mutex::new(rx);

    // rx is now `Clone`
    let rx = Arc::new(rx);

    let mut workers = Vec::with_capacity(pool_size);

    for id in 0..pool_size {
      // NOTE: This clones the Arc, not the reciever itself. (You can't clone the reciever)
      workers.push(Worker::new(id, Arc::clone(&rx)));
    }

    println!("{:#?}", workers);

    ThreadPool {
      workers,
      tx,
    }
  }

  pub fn execute<T>(&self, closure: T)
  where T: FnOnce() + Send + 'static {
    let job = Box::new(closure);

    self.tx.send(job).unwrap();
  }
}

impl Default for ThreadPool {
  /// The default size for the thread pool is one thread per logical CPU.
  fn default() -> ThreadPool {
    let logical_cpus = num_cpus::get();

    ThreadPool::new(logical_cpus)
  }
}
