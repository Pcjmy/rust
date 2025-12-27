pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = nums.len() as i32 - 1;
        while l < r {
            let mid = (l + r + 1) / 2;
            if nums[mid as usize] > target {
                r = mid - 1;
            } else {
                l = mid;
            }
        }
        if nums[l as usize] < target {
            return l + 1;
        }
        return l;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 3, 5, 6];
    let target1 = 5;
    println!("示例 1 - 输入: nums = {:?}, target = {}", nums1, target1);
    let result1 = Solution::search_insert(nums1, target1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![1, 3, 5, 6];
    let target2 = 2;
    println!("示例 2 - 输入: nums = {:?}, target = {}", nums2, target2);
    let result2 = Solution::search_insert(nums2, target2);
    println!("示例 2 - 输出: {}", result2);

    // 示例 3
    let nums3 = vec![1, 3, 5, 6];
    let target3 = 7;
    println!("示例 3 - 输入: nums = {:?}, target = {}", nums3, target3);
    let result3 = Solution::search_insert(nums3, target3);
    println!("示例 3 - 输出: {}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_insert_example_1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        let expected = 2;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    #[test]
    fn test_search_insert_example_2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        let expected = 1;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    #[test]
    fn test_search_insert_example_3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        let expected = 4;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    #[test]
    fn test_search_insert_target_not_in_array() {
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        let expected = 0;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    #[test]
    fn test_search_insert_single_element_found() {
        let nums = vec![1];
        let target = 1;
        let expected = 0;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    #[test]
    fn test_search_insert_single_element_insert_end() {
        let nums = vec![1];
        let target = 2;
        let expected = 1;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }

    #[test]
    fn test_search_insert_single_element_insert_beginning() {
        let nums = vec![3];
        let target = 2;
        let expected = 0;
        assert_eq!(Solution::search_insert(nums, target), expected);
    }
}