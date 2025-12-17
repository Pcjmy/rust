pub struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut i = 0;
        let mut j = 0;
        let len = nums.len();
        while j < len {
            if nums[j] != 0 {
                nums.swap(i, j);
                i += 1;
            }
            j += 1;
        }
    }
}

fn main() {
    // 示例 1
    let mut nums1 = vec![0, 1, 0, 3, 12];
    println!("示例 1 - 输入: {:?}", nums1);
    Solution::move_zeroes(&mut nums1);
    println!("示例 1 - 输出: {:?}", nums1);

    // 示例 2
    let mut nums2 = vec![0];
    println!("示例 2 - 输入: {:?}", nums2);
    Solution::move_zeroes(&mut nums2);
    println!("示例 2 - 输出: {:?}", nums2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_zeroes_basic_example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        let expected = vec![1, 3, 12, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_basic_example_2() {
        let mut nums = vec![0];
        let expected = vec![0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_empty_array() {
        let mut nums = vec![];
        let expected = vec![];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_single_element_nonzero() {
        let mut nums = vec![5];
        let expected = vec![5];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_all_zeros() {
        let mut nums = vec![0, 0, 0, 0];
        let expected = vec![0, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_no_zeros() {
        let mut nums = vec![1, 2, 3, 4];
        let expected = vec![1, 2, 3, 4];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_zeros_at_beginning() {
        let mut nums = vec![0, 0, 1, 2];
        let expected = vec![1, 2, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_zeros_at_end() {
        let mut nums = vec![1, 2, 0, 0];
        let expected = vec![1, 2, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_alternating_zeros() {
        let mut nums = vec![0, 1, 0, 2, 0, 3];
        let expected = vec![1, 2, 3, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_move_zeroes_many_zeros() {
        let mut nums = vec![0, 0, 0, 1, 2];
        let expected = vec![1, 2, 0, 0, 0];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, expected);
    }
}