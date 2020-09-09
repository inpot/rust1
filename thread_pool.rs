
use std::sync::Mutex;
use std::sync::Arc;
use std::thread;
use crate::mpsc::{Sender,Receiver, channel};

pub struct ThreadPool{
  size:u32,
   threads: Vec<Worker>,
   sender:Sender<Message> 
}

impl  ThreadPool {
pub  fn new(size :u32)->ThreadPool{
       assert!(size > 0);
       let mut threads = Vec::with_capacity(size as usize);

       let (sender,receiver) = channel();
        let receiver = Arc::new(Mutex::new(receiver));
        
        for i in 0..size {
          let  receiv = Arc::clone(&receiver);
          let name = format!("ThreadPool-{}",i);
          threads.push(Worker::new(receiv,name))
        }
    ThreadPool{
      size,
    threads,
    sender
    
    }
  }
   pub fn execute<F>(&self, f: F)
    where
            F: FnOnce() + Send + 'static
    {
      self.sender.send(Message::Work(Box::new(f))).unwrap();

    } 

}

struct Worker{
  thread: thread::JoinHandle<()> 
}
type Job = Box<dyn FnOnce() + Send + 'static>;

impl Worker{
  fn new(receiver:Arc<Mutex<Receiver<Message>>>,name:String)->Worker{
      let thread = thread::Builder::new().name(name).spawn(
        move || {
                let id = thread::current().name().unwrap_or("Defalut id Name").to_string();
            loop {
                let msg = receiver.lock().unwrap().recv().unwrap(); 
                match msg {
                  Message::Terminate =>{
                println!("Worker {} got a Terminate signal; Exist.",id); 
                    break;
                  }
                  Message::Work(job) =>{
                println!("Worker {} got a job; executing.",id); 
                job();

                  }



                }
            }
        }).unwrap(); 
        Worker {
            thread,
        } 
  } 
}

enum Message{
    Work(Job),
    Terminate, 
}