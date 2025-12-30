pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let sum: i32 = nums.iter().sum();
        if sum % 2 != 0 {
            return false;
        }
        let target = sum / 2;
        let mut f = vec![false; target as usize + 1];
        f[0] = true;
        for num in nums {
            for i in (num as usize..=target as usize).rev() {
                f[i] = f[i] || f[i - num as usize];
            }
        }
        return f[target as usize];
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 5, 11, 5];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::can_partition(nums1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![1, 2, 3, 5];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::can_partition(nums2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_partition_example_1() {
        let nums = vec![1, 5, 11, 5];
        let result = Solution::can_partition(nums);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_can_partition_example_2() {
        let nums = vec![1, 2, 3, 5];
        let result = Solution::can_partition(nums);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_can_partition_single_element() {
        let nums = vec![1];
        let result = Solution::can_partition(nums);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_can_partition_two_elements_equal() {
        let nums = vec![2, 2];
        let result = Solution::can_partition(nums);
        let expected = true;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_can_partition_two_elements_unequal() {
        let nums = vec![1, 2];
        let result = Solution::can_partition(nums);
        let expected = false;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_can_partition_empty() {
        let nums = vec![];
        let result = Solution::can_partition(nums);
        let expected = true; // Empty can be split into two empty sets
        assert_eq!(result, expected);
    }

    #[test]
    fn test_can_partition_odd_sum() {
        let nums = vec![1, 2, 3, 4, 5]; // Sum is 15, which is odd
        let result = Solution::can_partition(nums);
        let expected = false;
        assert_eq!(result, expected);
    }
}