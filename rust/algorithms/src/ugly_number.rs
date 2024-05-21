impl Solution {
    pub fn minimal_k_sum(nums: Vec<i32>, k: i32) -> i64 {
        let mut n =1;
        let mut max_val = nums;
        for 0 .. in k {
            if n < nums[0] {
               max_val = n.push(nums);
            } else {
                n += 2;
            }
            n += 1;
        }

        let mut m =0;
        let mut i =0;
        for 0 .. in i {
            m += max_val[i];
            i += 1;
        }
        m
    }
}