pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }
    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit() {
        assert_eq!(max_profit(vec![7,1,5,3,6,4]), 7);
        assert_eq!(max_profit(vec![1,2,3,4,5]), 4);
        assert_eq!(max_profit(vec![7,6,4,3,1]), 0);
        assert_eq!(max_profit(vec![1,2]), 1);
        assert_eq!(max_profit(vec![2,1]), 0);
    }
}
