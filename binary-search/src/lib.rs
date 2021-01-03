use core::cmp::Ord;
use std::cmp::Ordering;

pub fn find<U, T>(array: U, key: T) -> Option<usize>
where
    U: AsRef<[T]>,
    T: Ord,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let middle_index = array.len() / 2;
    let middle = &array[middle_index];
    let (left, right) = array.split_at(middle_index);

    match key.cmp(middle) {
        Ordering::Equal => Some(middle_index),
        Ordering::Less => find(left, key),
        Ordering::Greater => {
            if let Some(found) = find(&right[1..], key) {
                Some(middle_index + 1 + found)
            } else {
                None
            }
        }
    }
}
