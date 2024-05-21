pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;
    let mut max_area = 0;

    while left < right {
        let h = height[left].min(height[right]);
        max_area = max_area.max(h * (right - left) as i32);

        while left < right && height[left] <= h { left += 1; }
        while left < right && height[right] <= h { right -= 1; }
    }

    max_area
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
        assert_eq!(max_area(vec![1,1]), 1);
        assert_eq!(max_area(vec![4,3,2,1,4]), 16);
        assert_eq!(max_area(vec![1,2,1]), 2);
    }
}
