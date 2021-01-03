use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let middle_index = array.len() / 2;
    let middle = array[middle_index];

    match key.cmp(&middle) {
        Ordering::Equal => Some(middle_index),
        Ordering::Less => find(&array[..middle_index], key),
        Ordering::Greater => {
            if let Some(found) = find(&array[(middle_index + 1)..], key) {
                Some(middle_index + 1 + found)
            } else {
                None
            }
        }
    }
}
