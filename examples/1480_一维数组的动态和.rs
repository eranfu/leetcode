//! [1480. 一维数组的动态和](https://leetcode-cn.com/problems/running-sum-of-1d-array/)

impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }
        nums
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(
        Solution::running_sum(vec![1, 1, 1, 1, 1]),
        vec![1, 2, 3, 4, 5]
    );
    assert_eq!(
        Solution::running_sum(vec![3, 1, 2, 10, 1]),
        vec![3, 4, 6, 16, 17]
    );
}
