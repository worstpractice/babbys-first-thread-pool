use crate::types::Job;
use mpsc::Receiver;
use std::{
  sync::{
    mpsc,
    Arc,
    Mutex,
  },
  thread,
  time,
};
use thread::JoinHandle;
use time::Instant;

#[derive(Debug)]
pub struct Worker {
  id: usize,
  handle: JoinHandle<()>,
}

impl Worker {
  pub fn new(id: usize, rx: Arc<Mutex<Receiver<Job>>>) -> Worker {
    let handle = thread::spawn(move || {
      loop {
        let job = rx.lock().unwrap().recv().unwrap();

        println!("Worker {} got a job; executing.", id);

        let before = Instant::now();

        job();

        let after = Instant::now();

        let delta_time = after - before;

        println!("Worker {} completed the task.", id);
        println!("Time elapsed: {:#?}", delta_time)
      }
    });

    Worker {
      id,
      handle,
    }
  }
}
