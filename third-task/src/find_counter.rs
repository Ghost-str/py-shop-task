use std::sync::RwLock;

/// Structure for tracking the number of elements found
pub struct FoundCounter {
    current: RwLock<u64>,
    max_prints: u64,
}

impl FoundCounter {
    pub fn new(max: u64) -> FoundCounter {
        FoundCounter {
            current: RwLock::new(0),
            max_prints: max,
        }
    }

    /// Checks if there is a place to find another element
    pub fn is_available(&self) -> bool {
        {
            let curr = *self.current.read().unwrap();
            curr < self.max_prints
        }
    }

    /// Checks if it is still possible to find one element and increments the internal counter of found
    pub fn is_can_print(&self) -> bool {
        {
            let mut val = self.current.write().unwrap();
            *val += 1;
            *val <= self.max_prints
        }
    }
}

#[test]
fn print_count() {
    let mut print_vect: Vec<bool> = Vec::new();

    let counter = FoundCounter::new(5);

    for _ in 0..6 {
        if counter.is_can_print() {
            print_vect.push(true);
        }
    }

    assert_eq!(print_vect.len(), 5);
}

#[test]
fn is_avalible_use_case() {
    let counter = FoundCounter::new(5);

    for _ in 0..5 {
        counter.is_can_print();
    }

    assert_eq!(counter.is_available(), false);
}
