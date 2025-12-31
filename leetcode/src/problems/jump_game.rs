pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reach = 0;
        for (i, &jump) in nums.iter().enumerate() {
            if i > max_reach {
                return false;
            }
            max_reach = max_reach.max(i + jump as usize);
        }
        return true;
    }
}

fn main() {
    // 示例 1：
    let nums1 = vec![2,3,1,1,4];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::can_jump(nums1);
    println!("示例 1 - 输出: {}", result1);
    println!("解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。");

    // 示例 2：
    let nums2 = vec![3,2,1,0,4];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::can_jump(nums2);
    println!("示例 2 - 输出: {}", result2);
    println!("解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_jump_example_1() {
        let nums = vec![2,3,1,1,4];
        assert_eq!(Solution::can_jump(nums), true);
    }

    #[test]
    fn test_can_jump_example_2() {
        let nums = vec![3,2,1,0,4];
        assert_eq!(Solution::can_jump(nums), false);
    }

    #[test]
    fn test_can_jump_single_element() {
        let nums = vec![0];
        assert_eq!(Solution::can_jump(nums), true);
    }

    #[test]
    fn test_can_jump_single_element_nonzero() {
        let nums = vec![1];
        assert_eq!(Solution::can_jump(nums), true);
    }

    #[test]
    fn test_can_jump_starts_with_zero() {
        let nums = vec![0,1];
        assert_eq!(Solution::can_jump(nums), false);
    }
}