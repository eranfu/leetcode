//! [1968. 构造元素不等于两相邻元素平均值的数组](https://leetcode-cn.com/problems/array-with-elements-not-equal-to-average-of-neighbors/)

impl Solution {
    pub fn rearrange_array(mut nums: Vec<i32>) -> Vec<i32> {
        let mid = nums.len() / 2;
        nums.sort_unstable();
        let mut res = Vec::with_capacity(nums.len());
        let mut left = 0;
        let mut right = mid;
        while right < nums.len() {
            res.push(nums[right]);
            right += 1;
            if left < mid {
                res.push(nums[left]);
                left += 1;
            }
        }
        res
    }
}

struct Solution;

fn is_correct(vec: &[i32]) -> bool {
    let mut i = 1;
    let last = vec.len() - 1;
    while i < last {
        if vec[i - 1] - vec[i] == vec[i] - vec[i + 1] {
            println!("{:?}", vec);
            return false;
        }
        i += 1
    }
    true
}

fn main() {
    assert!(is_correct(&Solution::rearrange_array(vec![1, 0, 2])));
    assert!(is_correct(&Solution::rearrange_array(vec![1, 3, 2])));
    assert!(is_correct(&Solution::rearrange_array(vec![1, 3, 5, 7])));
    assert!(is_correct(&Solution::rearrange_array(vec![1, 2, 3])));
    assert!(is_correct(&Solution::rearrange_array(vec![1, 2, 3, 4, 5])));
    assert!(is_correct(&Solution::rearrange_array(vec![6, 2, 0, 9, 7])));
}
