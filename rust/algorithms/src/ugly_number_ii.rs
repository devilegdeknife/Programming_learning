pub fn nth_ugly_number(n: i32) -> i32 {
    let mut ugly_numbers = vec![1];
    let (mut i2, mut i3, mut i5) = (0, 0, 0);
    for _ in 1..n as usize {
        let next = *[
            ugly_numbers[i2] * 2,
            ugly_numbers[i3] * 3,
            ugly_numbers[i5] * 5,
        ]
            .iter()
            .min()
            .unwrap();
        ugly_numbers.push(next);
        if ugly_numbers[i2] * 2 == next {
            i2 += 1;
        }
        if ugly_numbers[i3] * 3 == next {
            i3 += 1;
        }
        if ugly_numbers[i5] * 5 == next {
            i5 += 1;
        }
    }
    *ugly_numbers.last().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nth_ugly_number() {
        assert_eq!(nth_ugly_number(1), 1);
        assert_eq!(nth_ugly_number(2), 2);
        assert_eq!(nth_ugly_number(3), 3);
        assert_eq!(nth_ugly_number(4), 4);
        assert_eq!(nth_ugly_number(5), 5);
        assert_eq!(nth_ugly_number(6), 6);
        assert_eq!(nth_ugly_number(7), 8);
        assert_eq!(nth_ugly_number(10), 12);
        assert_eq!(nth_ugly_number(11), 15);
        assert_eq!(nth_ugly_number(15), 24);
        assert_eq!(nth_ugly_number(1407), 536870912);
    }
}
