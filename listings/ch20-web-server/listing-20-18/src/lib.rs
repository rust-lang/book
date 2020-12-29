use std::sync::mpsc;
use std::thread;
// ANCHOR: here
use std::sync::Arc;
use std::sync::Mutex;
// --snip--

// ANCHOR_END: here
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

// ANCHOR: here
impl ThreadPool {
    // --snip--
    // ANCHOR_END: here
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    // ANCHOR: here
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    // --snip--
    // ANCHOR_END: here

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
    // ANCHOR: here
}

// --snip--

// ANCHOR_END: here
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

// ANCHOR: here
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // --snip--
        // ANCHOR_END: here
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
        // ANCHOR: here
    }
}
// ANCHOR_END: here
