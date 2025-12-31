pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for num in nums {
            result ^= num;
        }

        return result;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![2, 2, 1];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::single_number(nums1.clone());
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![4, 1, 2, 1, 2];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::single_number(nums2.clone());
    println!("示例 2 - 输出: {}", result2);

    // 示例 3
    let nums3 = vec![1];
    println!("示例 3 - 输入: {:?}", nums3);
    let result3 = Solution::single_number(nums3.clone());
    println!("示例 3 - 输出: {}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_number_example_1() {
        let nums = vec![2, 2, 1];
        let expected = 1;
        let result = Solution::single_number(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_number_example_2() {
        let nums = vec![4, 1, 2, 1, 2];
        let expected = 4;
        let result = Solution::single_number(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_number_example_3() {
        let nums = vec![1];
        let expected = 1;
        let result = Solution::single_number(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_number_single_element() {
        let nums = vec![42];
        let expected = 42;
        let result = Solution::single_number(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_number_negative_numbers() {
        let nums = vec![-1, -1, -2];
        let expected = -2;
        let result = Solution::single_number(nums);
        assert_eq!(result, expected);
    }
}