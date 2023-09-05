use std::sync::RwLock;

pub struct PrintCount {
    curr: RwLock<u64>,
    max_prints: u64,
}

impl PrintCount {
    pub fn new(max: u64) -> PrintCount {
        PrintCount {
            curr: RwLock::new(0),
            max_prints: max,
        }
    }

    pub fn is_avalible(&self) -> bool {
        {
            let curr = *self.curr.read().unwrap();
            curr < self.max_prints
        }
    }

    pub fn is_can_print(&self) -> bool {
        {
            let mut val = self.curr.write().unwrap();
            *val += 1;
            *val <= self.max_prints
        }
    }
}

#[test]
fn print_count() {
    let mut print_vect: Vec<bool> = Vec::new();

    let counter = PrintCount::new(5);

    for _ in 0..6 {
        if counter.is_can_print() {
            print_vect.push(true);
        }
    }

    assert_eq!(print_vect.len(), 5);
}

#[test]
fn is_avalible_use_case() {
    let counter = PrintCount::new(5);

    for _ in 0..5 {
        counter.is_can_print();
    }

    assert_eq!(counter.is_avalible(), false);
}
