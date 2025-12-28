pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        use std::cmp::max;

        let len = nums.len();
        let mut f = vec![0; len];
        for i in 0..len {
            if i == 0 {
                f[i] = nums[i];
            } else if i == 1 {
                f[i] = max(nums[i], f[(i - 1) as usize]);
            } else {
                f[i] = max(f[(i - 1) as usize], f[(i - 2) as usize] + nums[i]);
            }
        }

        return f[(len as i32 - 1) as usize];
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1,2,3,1];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::rob(nums1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![2,7,9,3,1];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::rob(nums2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rob_example_1() {
        let nums = vec![1,2,3,1];
        let expected = 4;
        assert_eq!(Solution::rob(nums), expected);
    }

    #[test]
    fn test_rob_example_2() {
        let nums = vec![2,7,9,3,1];
        let expected = 12;
        assert_eq!(Solution::rob(nums), expected);
    }

    #[test]
    fn test_rob_single_house() {
        let nums = vec![5];
        let expected = 5;
        assert_eq!(Solution::rob(nums), expected);
    }

    #[test]
    fn test_rob_two_houses() {
        let nums = vec![2, 3];
        let expected = 3;
        assert_eq!(Solution::rob(nums), expected);
    }
}