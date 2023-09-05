mod check_hash;
mod hash;
mod number_iterator;
mod read_args;

use check_hash::fast_check_hash;
use hash::Hash;
use number_iterator::NumberIterator;
use read_args::get_args;
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};

fn main() {
    let args = get_args();
    let shared_number_generator = Arc::new(Mutex::new(NumberIterator::new(0)));
    let print_count = Arc::new(RwLock::new(0));

    let mut workers = vec![];
    for _ in 0..num_cpus::get() {
        let clone = shared_number_generator.clone();
        let print_count_clon = print_count.clone();
        let thread = thread::spawn(move || loop {
            let curr_count = { *print_count_clon.read().unwrap() };
            if curr_count >= args.find {
                break;
            }

            let current_number: Option<u128> = {
                let val = clone.lock().unwrap().next();
                val
            };

            match current_number {
                None => break,
                Some(number) => {
                    let hash = Hash::from(number);
                    if fast_check_hash(&hash, args.number) {
                        let current = {
                            let mut val = print_count_clon.write().unwrap();
                            *val += 1;
                            *val
                        };

                        if current > args.find {
                            break;
                        }
                        println!("{number} {}", hash);
                    }
                }
            }
        });

        workers.push(thread);
    }

    for thread in workers {
        let _ = thread.join();
    }
}
