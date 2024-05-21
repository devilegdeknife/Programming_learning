use std::cmp::{max, min};

pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut min_right = arr.clone();
    for i in (0..arr.len() - 1).rev() {
        min_right[i] = min(min_right[i + 1], arr[i]);
    }

    let mut chunks = 1;
    let mut max_left = arr[0];
    for i in 0..arr.len() - 1 {
        max_left = max(max_left, arr[i]);
        if max_left <= min_right[i + 1] {
            chunks += 1;
        }
    }

    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_chunks_to_sorted() {
        assert_eq!(max_chunks_to_sorted(vec![4,3,2,1,0]), 1);
        assert_eq!(max_chunks_to_sorted(vec![1,0,2,3,4]), 4);
        assert_eq!(max_chunks_to_sorted(vec![2,1,3,4,4]), 4);
    }
}
