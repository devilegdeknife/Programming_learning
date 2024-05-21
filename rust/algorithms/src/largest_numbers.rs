pub fn largest_number(nums: Vec<i32>) -> String {
    let mut nums: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();
    nums.sort_unstable_by(|a, b| (b.clone() + a).cmp(&(a.clone() + b)));
    if nums[0] == "0" {
        return "0".to_string();
    }
    nums.join("")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_number() {
        assert_eq!(largest_number(vec![10, 2]), "210".to_string());
        assert_eq!(largest_number(vec![3, 30, 34, 5, 9]), "9534330".to_string());
        assert_eq!(largest_number(vec![1]), "1".to_string());
        assert_eq!(largest_number(vec![10]), "10".to_string());
        assert_eq!(largest_number(vec![0, 0]), "0".to_string());
    }
}
