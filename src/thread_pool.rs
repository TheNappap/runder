use std::thread;
use std::sync::mpsc::{self,Sender,Receiver};
use std::sync::Arc;
use std::sync::Mutex;

trait FnBox {
    fn call(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call(self: Box<F>) {
        (*self)()
    }
}

type Job = Box<FnBox + Send + 'static>;

enum ThreadMessage {
    NewJob(Job),
    Terminate
}

struct Thread {
    index: usize,
    thread_join: Option<thread::JoinHandle<()>>
}

impl Thread {
    fn new(index: usize, receiver: Arc<Mutex<Receiver<ThreadMessage>>>) -> Thread {
        let thread_join = thread::spawn( move|| {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    ThreadMessage::NewJob(job) =>{
                        //println!("Thread {} started executing job.", index);
                        job.call();
                        //println!("Thread {} finished job.", index);
                    },
                    ThreadMessage::Terminate => {
                        //println!("Thread {} terminated.", index);
                        break;
                    }
                }
            }
        });
        Thread{index, thread_join: Some(thread_join) }
    }
}

pub struct ThreadPool {
    threads: Vec<Thread>,
    sender: Sender<ThreadMessage>
}

impl ThreadPool {
    pub fn new(amt_threads: usize) -> ThreadPool {
        if amt_threads == 0 { panic!("There should be at least one thread in a thread pool.") }

        let mut threads = Vec::with_capacity(amt_threads);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for index in 0..amt_threads {
            let thread = Thread::new(index, receiver.clone());
            threads.push(thread);
        }
        ThreadPool{threads, sender}
    }

    pub fn amt_threads(&self) -> usize {
        self.threads.len()
    }

    pub fn execute<F>(&self, job: F) where F : FnOnce() + Send + 'static {
        let job = Box::new(job);

        self.sender.send(ThreadMessage::NewJob(job)).unwrap();
    }

    pub fn terminate_thread(&self, index: usize) {
        self.sender.send(ThreadMessage::Terminate).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.threads {
            self.sender.send(ThreadMessage::Terminate).unwrap();
        }

        for thread in &mut self.threads {
            if let Some(thread) = thread.thread_join.take() {
                thread.join().unwrap();
            }
        }
    }
}