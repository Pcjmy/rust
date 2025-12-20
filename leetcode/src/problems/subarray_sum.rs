pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut ans = 0;
        let mut sum = 0;
        let len = nums.len();
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);

        for i in 0..len {
            sum += nums[i];
            if map.contains_key(&(sum - k)) {
                ans += map[&(sum - k)];
            }
            *map.entry(sum).or_insert(0) += 1;
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 1, 1];
    let k1 = 2;
    println!("示例 1 - 输入: nums = {:?}, k = {}", nums1, k1);
    let result1 = Solution::subarray_sum(nums1, k1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![1, 2, 3];
    let k2 = 3;
    println!("示例 2 - 输入: nums = {:?}, k = {}", nums2, k2);
    let result2 = Solution::subarray_sum(nums2, k2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subarray_sum_example_1() {
        let nums = vec![1, 1, 1];
        let k = 2;
        let expected = 2; // Based on the example
        let result = Solution::subarray_sum(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_subarray_sum_example_2() {
        let nums = vec![1, 2, 3];
        let k = 3;
        let expected = 2; // Based on the example
        let result = Solution::subarray_sum(nums, k);
        assert_eq!(result, expected);
    }
}