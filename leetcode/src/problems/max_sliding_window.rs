pub struct Solution;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::BinaryHeap;

        let mut ans = Vec::new();
        let len = nums.len();
        let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();

        for i in 0..k as usize {
            heap.push((nums[i], i as i32));
        }

        for i in (k as usize -1)..len {
            let pos = i as i32 - k;
            if i >= k as usize {
                heap.push((nums[i], i as i32));
            }
            while let Some(&(x, y)) = heap.peek() {
                if y > pos {
                    ans.push(x);
                    break;
                }
                heap.pop();
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