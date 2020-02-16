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
    Finish(Job),
    Terminate
}

struct Thread {
    _index: usize,
    thread_join: Option<thread::JoinHandle<()>>
}

impl Thread {
    fn new(index: usize, receiver: Arc<Mutex<Receiver<ThreadMessage>>>) -> Thread {
        let thread_join = thread::spawn( move|| {
            loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    ThreadMessage::NewJob(job) =>{
                        job.call();
                    },
                    ThreadMessage::Finish(job) => {
                        job.call();
                        break;
                    },
                    ThreadMessage::Terminate => {
                        break;
                    }
                }
            }
        });
        Thread{_index: index, thread_join: Some(thread_join) }
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

        if let Err(e) = self.sender.send(ThreadMessage::NewJob(job)) {
            println!("{:?}", e);
        }
    }

    pub fn finish<F>(&self, job: F) where F : FnOnce() + Send + 'static {
        let job = Box::new(job);

        if let Err(e) = self.sender.send(ThreadMessage::Finish(job)) {
            println!("{:?}", e);
        }
    }

    pub fn finish_jobs(self) {
        thread::spawn(move ||{
           let _sink = self;
        });
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.threads {
            if let Err(e) = self.sender.send(ThreadMessage::Terminate) {
                println!("{:?}", e);
            }
        }

        for thread in &mut self.threads {
            if let Some(thread) = thread.thread_join.take() {
                if let Err(e) = thread.join() {
                    println!("{:?}", e);
                }
            }
        }
    }
}