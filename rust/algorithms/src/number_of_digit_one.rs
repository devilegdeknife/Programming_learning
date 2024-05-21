pub fn count_digit_one(n: i32) -> i32 {
    let n = n as i64;
    let mut count = 0;
    let mut factor = 1;
    while n / factor != 0 {
        let digit = (n / factor) % 10;
        let higher = n / (factor * 10);
        let lower = n - (n / factor) * factor;
        count += higher * factor;
        if digit == 1 {
            count += lower + 1;
        } else if digit > 1 {
            count += factor;
        }
        factor *= 10;
    }
    count as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_digit_one() {
        assert_eq!(count_digit_one(0), 0);
        assert_eq!(count_digit_one(1), 1);
        assert_eq!(count_digit_one(10), 2);
        assert_eq!(count_digit_one(13), 6);
        assert_eq!(count_digit_one(20), 12);
        assert_eq!(count_digit_one(55), 16);
        assert_eq!(count_digit_one(99), 20);
        assert_eq!(count_digit_one(100), 21);
    }
}
