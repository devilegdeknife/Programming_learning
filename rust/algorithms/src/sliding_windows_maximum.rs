mod sliding_windows_maximum {
    use std::collections::VecDeque;

    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut deque = VecDeque::new();
        let mut result = vec![];

        for i in 0..nums.len() {
            // 当前元素大于队尾元素时，不断弹出队尾元素
            while let Some(&last) = deque.back() {
                if nums[i] > nums[last] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);  // 将当前元素索引放入队列尾部

            // 当队列头部元素索引超出窗口范围时，弹出队列头部元素
            if i >= k && deque[0] <= i - k {
                deque.pop_front();
            }

            // 当窗口形成时，将队列头部元素（窗口最大值）放入结果数组
            if i >= k - 1 {
                result.push(nums[deque[0]]);
            }
        }

        result
    }
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_max_sliding_window() {
            let nums = vec![1,3,-1,-3,5,3,6,7];
            let k = 3;
            let expected = vec![3, 3, 5, 5, 6, 7];
            assert_eq!(max_sliding_window(nums, k), expected);
        }
    }

}