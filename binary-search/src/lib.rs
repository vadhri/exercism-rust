
pub fn search(array: &[i32], key: i32, parent_array_index: i32) -> i32 {
    let mid = array.len() / 2;

    if array.len() == 0 || (array.len() == 1 && array[0] != key) {
        -1
    } else if array[mid] == key {
        parent_array_index + mid as i32
    } else if array[mid] > key {
        search(&array[0..mid], key, parent_array_index)
    } else {
        search(&array[mid..array.len()], key, parent_array_index + mid as i32)
    }
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let index = search(array, key, 0);

    match index {
        -1 => None,
        _ => Some(index as usize)
    }
}
