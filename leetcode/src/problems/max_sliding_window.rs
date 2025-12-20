pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::VecDeque;

        let mut deque = VecDeque::new();
        let mut ans = Vec::new();
        let len = nums.len();

        for i in 0..len {
            while let Some(&back_val) = deque.back() {
                if nums[i] >= nums[back_val] {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back(i);
            while let Some(&front_val) = deque.front() {
                if front_val as i32 <= i as i32 - k {
                    deque.pop_front();
                } else {
                    break;
                }
            }
            if i >= k as usize -1 {
                if let Some(&front_val) = deque.front() {
                    ans.push(nums[front_val]);
                }
            }
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1,3,-1,-3,5,3,6,7];
    let k1 = 3;
    println!("示例 1 - 输入: nums = {:?}, k = {}", nums1, k1);
    let result1 = Solution::max_sliding_window(nums1, k1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let nums2 = vec![1];
    let k2 = 1;
    println!("示例 2 - 输入: nums = {:?}, k = {}", nums2, k2);
    let result2 = Solution::max_sliding_window(nums2, k2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sliding_window_basic_example_1() {
        let nums = vec![1,3,-1,-3,5,3,6,7];
        let k = 3;
        let expected = vec![3,3,5,5,6,7];
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_sliding_window_basic_example_2() {
        let nums = vec![1];
        let k = 1;
        let expected = vec![1];
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_sliding_window_edge_case() {
        let nums = vec![1, -1];
        let k = 1;
        let expected = vec![1, -1];
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_sliding_window_negative_numbers() {
        let nums = vec![-7, -8, 7, 5, 7, 1, 6, 0];
        let k = 4;
        let expected = vec![7, 7, 7, 7, 7];
        let result = Solution::max_sliding_window(nums, k);
        assert_eq!(result, expected);
    }
}