use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub mod add_millimeters_to_meters;
pub mod advanced_match;
pub mod algorithms;
pub mod api_mutex;
pub mod arrays;
pub mod basics;
pub mod boxes;
pub mod closures;
pub mod collections;
pub mod custom_smart_pointer;
pub mod data_collections;
pub mod decisions;
pub mod destructuring_structs;
pub mod doc;
pub mod draw;
pub mod enums;
pub mod errors;
pub mod exercises;
pub mod expressions;
pub mod extern_functions;
pub mod external_crates;
pub mod futures;
pub mod generics;
pub mod get_type;
pub mod hashmaps;
pub mod iterators;
pub mod joinhandle;
pub mod lifetimes;
pub mod loops;
pub mod match_expression;
pub mod message_passing;
pub mod messenger;
pub mod methods;
pub mod modules;
pub mod move_vars;
pub mod mutable_reference;
pub mod object_oriented;
pub mod operator_overloading;
pub mod optionals;
pub mod ownership;
pub mod patterns;
pub mod post;
pub mod print_var_type;
pub mod random_numbers;
pub mod ranges;
pub mod rc_t;
pub mod references;
pub mod results;
pub mod static_variable;
pub mod strings;
pub mod structs;
pub mod testing;
pub mod threads;
pub mod traits;
pub mod tree_data_structure;
pub mod tuples;
pub mod unsafe_code;
pub mod unsafe_trait;
pub mod user_input;
pub mod vectors;
pub mod webserver;
pub mod while_loop;

pub fn print_title(title: &str) {
    println!(" ");
    println!("### {title} ###");
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
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

    pub fn execute<F>(&self, _f: F)
    where
        F: FnOnce() + Send + 'static,
    {}
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| {
            receiver;
        });

        Worker { id, thread }
    }
}

/*pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
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

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(|| receiver);

        Worker { id, thread }
    }
}
*/