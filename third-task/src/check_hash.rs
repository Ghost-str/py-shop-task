use crate::hash::Hash;

pub fn fast_check_hash(hash: &Hash, count: u8) -> bool {
    let is_even = count % 2 == 0;
    let array_check = count / 2;

    for index in 0..array_check {
        let index_from_end = 31 - index;
        let value = hash[index_from_end];
        if value != 0 {
            return false;
        }
    }

    if !is_even {
        let index = 31 - array_check;
        let val = hash[index];
        return (val & 0b0000_1111) == 0;
    }

    true
}

#[test]
fn check_hash_not_even_zero() {
    let val = Hash::from(51);

    assert_eq!(fast_check_hash(&val, 1), true);
}

#[test]
fn check_hash_even_zero() {
    let val = Hash::from(748);

    assert_eq!(fast_check_hash(&val, 2), true);
}
