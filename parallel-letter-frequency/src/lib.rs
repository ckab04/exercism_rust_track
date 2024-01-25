use std::{collections::HashMap, thread, sync::{mpsc, Arc, Mutex}};

struct ThreadPool{
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}



type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool{
    fn new(size: usize)-> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size{
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool{workers, sender}

    }

    fn execute<F>(&self, f: F) 
    where 
    F: FnOnce() + Send + 'static,{
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

//struct Job;

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{

    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{

        let thread = thread::spawn(move || loop {

            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job: executing...");

            job();

        });
        Worker{id, thread}
    }
}


pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let global_map = HashMap::new();


    /*
    (
        "Count the frequency of letters in the given input '{input:?}'. Ensure that you are using {} to process the input.",
        match worker_count {
            1 => "1 worker".to_string(),
            _ => format!("{worker_count} workers"),
        }
    );*/

    global_map
}


fn parallel_frequency(letter: &str)-> HashMap<char, usize>{

let mut frequencies = HashMap::new();
letter.chars().for_each(|c| {
    if frequencies.contains_key(&c){
       let value = *frequencies.get(&c).expect("Could not get the value");
       frequencies.insert(c, value + 1);
    }else{
        frequencies.insert(c, 1);
    }
});

frequencies
}


