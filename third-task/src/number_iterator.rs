
/// Iterator for get next value for hash check
pub struct NumberIterator {
    value: u128,
    shift: u128,
}

impl NumberIterator {
    pub fn new(start_value: u128, shift: usize) -> NumberIterator {
        NumberIterator {
            value: start_value,
            shift: shift as u128,
        }
    }
}

impl Iterator for NumberIterator {
    type Item = u128;

    fn next(&mut self) -> Option<Self::Item> {
        let (value, is_overflow) = self.value.overflowing_add(1 + self.shift);

        if is_overflow {
            return None;
        }

        self.value = value;

        Some(value)
    }
}
