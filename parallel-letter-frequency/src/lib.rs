use std::{collections::HashMap, thread};

struct ThreadPool{
    workers: Vec<Worker>,
}

impl ThreadPool{
    fn new(size: usize)-> ThreadPool{
        assert!(size > 0);
        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            workers.push(Worker::new(id));
        }

        ThreadPool{workers}

    }
}

struct Worker{
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker{

    fn new(id: usize) -> Worker{

        let thread = thread::spawn(|| {});
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
