use std::sync::Mutex;
use std::thread;
use std::{fmt::Display, sync::Arc};

use crate::check_hash::fast_check_hash;
use crate::find_counter::FoundCounter;
use crate::hash::Hash;
use crate::number_iterator::NumberIterator;

/// Container for founded hash
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoundHash {
    pub number: u128,
    pub hash: Hash,
}

impl Display for FoundHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.number, self.hash)
    }
}

impl From<u128> for FoundHash {
    fn from(value: u128) -> Self {
        FoundHash {
            number: value,
            hash: Hash::from(value),
        }
    }
}

/// Function for search hashes
pub fn find_hashes(number: u8, find: u64) -> Vec<FoundHash> {
    let print_count = Arc::new(FoundCounter::new(find));
    let result_hash_vec = Arc::new(Mutex::new(Vec::new()));

    let mut workers = vec![];

    let cpu_count = num_cpus::get();

    for cpu_index in 0..cpu_count {
        let print_count_clon = print_count.clone();
        let result_hash_vec_clon = result_hash_vec.clone();

        let thread = thread::spawn(move || {
            let mut number_iter = NumberIterator::new(cpu_index as u128, cpu_count);

            loop {
                if !print_count_clon.is_available() {
                    break;
                }

                let current_number: Option<u128> = number_iter.next();

                match current_number {
                    None => break,
                    Some(current_number) => {
                        let hash = Hash::from(current_number);
                        if fast_check_hash(&hash, number) {
                            if !print_count_clon.is_can_print() {
                                break;
                            }

                            {
                                result_hash_vec_clon.lock().unwrap().push(FoundHash {
                                    number: current_number,
                                    hash,
                                });
                            }
                        }
                    }
                }
            }
        });

        workers.push(thread);
    }

    for thread in workers {
        let _ = thread.join();
    }

    let mutex = Arc::try_unwrap(result_hash_vec).unwrap();
    mutex.into_inner().unwrap()
}

#[test]
fn find_hashes_test() {
    let expected = vec![
        FoundHash::from(178116),
        FoundHash::from(483338),
        FoundHash::from(1087996),
        FoundHash::from(3944120),
        FoundHash::from(5372565),
    ];

    let result = find_hashes(5, 5);

    assert_eq!(expected, result);
}
