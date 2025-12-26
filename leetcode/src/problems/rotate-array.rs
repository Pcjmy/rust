pub struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        if k == 0 {
            return ;
        }
        let arr = nums.clone();
        let x = nums[0];
        let len = nums.len();
        let mut cur = 0;
        for i in 0..len {
            let pos = (i + k as usize) % len;
            nums[pos] = arr[i];
        }
    }
}

fn main() {
    // 示例 1
    let mut nums1 = vec![1, 2, 3, 4, 5, 6, 7];
    let k1 = 3;
    println!("Rotate Array - 示例 1 - 输入: nums = {:?}, k = {}", nums1, k1);
    Solution::rotate(&mut nums1, k1);
    println!("Rotate Array - 示例 1 - 输出: {:?}", nums1);

    // 示例 2
    let mut nums2 = vec![-1, -100, 3, 99];
    let k2 = 2;
    println!("Rotate Array - 示例 2 - 输入: nums = {:?}, k = {}", nums2, k2);
    Solution::rotate(&mut nums2, k2);
    println!("Rotate Array - 示例 2 - 输出: {:?}", nums2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_array_basic_example_1() {
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        let expected = vec![5, 6, 7, 1, 2, 3, 4];
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_rotate_array_basic_example_2() {
        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        let expected = vec![3, 99, -1, -100];
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_rotate_array_single_element() {
        let mut nums = vec![1];
        let k = 1;
        let expected = vec![1];
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_rotate_array_k_greater_than_length() {
        let mut nums = vec![1, 2];
        let k = 3; // k > nums.len()
        let expected = vec![2, 1]; // equivalent to k = 1
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_rotate_array_k_is_zero() {
        let mut nums = vec![1, 2, 3];
        let k = 0;
        let expected = vec![1, 2, 3];
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, expected);
    }
}