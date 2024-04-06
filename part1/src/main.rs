use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::fs::{OpenOptions};
use std::io::Write;
use rand::{distributions::{Uniform, Distribution}, thread_rng};
use rand::Rng;

struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

struct LinkedList {
    head: Mutex<Option<Box<Node>>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: Mutex::new(None) }
    }

    fn insert(&self, value: i32) {
        let mut new_node = Box::new(Node { data: value, next: None });
        let mut current = self.head.lock().unwrap();
    
        if current.is_none() || current.as_ref().unwrap().data > value {
            new_node.next = current.take();
            *current = Some(new_node);
            return;
        }
    
        let mut prev = current.as_mut().unwrap();
        while prev.next.is_some() && prev.next.as_ref().unwrap().data < value {
            prev = prev.next.as_mut().unwrap();
        }
        new_node.next = prev.next.take();
        prev.next = Some(new_node);
    }
    

    fn delete(&self, value: i32) -> bool {
        let mut current = self.head.lock().unwrap();

        if current.is_none() {
            return false;
        }

        if current.as_ref().unwrap().data == value {
            *current = current.take().unwrap().next;
            return true;
        }

        let mut prev = current.as_mut().unwrap();
        while prev.next.is_some() && prev.next.as_ref().unwrap().data != value {
            prev = prev.next.as_mut().unwrap();
        }

        if prev.next.is_some() {
            prev.next = prev.next.take().unwrap().next;
            return true;
        }

        false
    }

    fn search(&self, value: i32) -> bool {
        let current = self.head.lock().unwrap();
        let mut node = current.as_ref();
        while let Some(n) = node {
            if n.data == value {
                return true;
            }
            node = n.next.as_ref();
        }
        false
    }
}

// fn main() {
//     let list = Arc::new(LinkedList::new());
//     let mut handles = vec![];
//     let range = Uniform::from(0..100000);

//     for _ in 0..4 {
//         let list_clone = Arc::clone(&list);
//         let handle = thread::spawn(move || {
//             let mut rng = thread_rng();
//             for _ in 0..25000 {
//                 let action = rng.gen_range(0..3);
//                 let value = range.sample(&mut rng);
//                 match action {
//                     0 => list_clone.insert(value),
//                     1 => { let _ = list_clone.delete(value); },
//                     _ => { let _ = list_clone.search(value); },
//                 }
//             }
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }
// }

fn main() {
    let list = Arc::new(LinkedList::new());
    let mut handles = vec![];
    let range = Uniform::from(0..500000); 

    let (log_sender, log_receiver) = mpsc::channel();

    let logger_handle = thread::spawn(move || {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .expect("Unable to open log file");

        for log in log_receiver {
            writeln!(file, "{}", log).expect("Unable to write to log file");
        }
    });

    for i in 0..4 { 
        let list_clone = Arc::clone(&list);
        let tx = log_sender.clone();
        let handle = thread::spawn(move || {
            let mut rng = thread_rng();
            for _ in 0..125000 {
                let action = rng.gen_range(0..3);
                let value = range.sample(&mut rng);
                let action_str = match action {
                    0 => "Inserting",
                    1 => "Deleting",
                    _ => "Searching for",
                };
                let log = format!("Thread {}: {} value: {}", i + 1, action_str, value);
                tx.send(log).unwrap();
                match action {
                    0 => {
                        list_clone.insert(value);
                    },
                    1 => {
                        let _ = list_clone.delete(value); 
                    },
                    _ => {
                        let _ = list_clone.search(value); 
                    },
                }
            }
            
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(log_sender);

    logger_handle.join().unwrap();
}
