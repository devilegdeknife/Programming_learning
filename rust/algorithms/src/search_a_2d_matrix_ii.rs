pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    if matrix.is_empty() || matrix[0].is_empty() {
        return false;
    }

    let mut row = 0;
    let mut col = matrix[0].len() as i32 - 1;

    while row < matrix.len() && col >= 0 {
        if matrix[row][col as usize] == target {
            return true;
        } else if matrix[row][col as usize] < target {
            row += 1;
        } else {
            col -= 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix() {
        let matrix = vec![
            vec![1, 4, 7, 11, 15],
            vec![2, 5, 8, 12, 19],
            vec![3, 6, 9, 16, 22],
            vec![10, 13, 14, 17, 24],
            vec![18, 21, 23, 26, 30]
        ];
        assert_eq!(search_matrix(matrix.clone(), 5), true);
        assert_eq!(search_matrix(matrix.clone(), 20), false);
    }
}
