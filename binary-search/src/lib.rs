use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let middle_index = array.len() / 2;
    let middle = array[middle_index];
    let (left, right) = array.split_at(middle_index);

    match key.cmp(&middle) {
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
