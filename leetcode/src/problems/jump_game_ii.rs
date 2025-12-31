pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 1 {
            return 0;
        }

        let mut jumps = 0;
        let mut current_end = 0;
        let mut farthest = 0;
        
        for i in 0..n - 1 {
            farthest = farthest.max(i + nums[i] as usize);
            
            if i == current_end {
                jumps += 1;
                current_end = farthest;
                
                if current_end >= n - 1 {
                    break;
                }
            }
        }
        
        return jumps;
    }
}

fn main() {
    // 示例 1:
    let nums1 = vec![2,3,1,1,4];
    println!("输入: nums = {:?}", nums1);
    let result1 = Solution::jump(nums1);
    println!("输出: {}", result1);
    println!("解释: 跳到最后一个位置的最小跳跃数是 {}。", result1);
    println!("     从下标为 0 跳到下标为 1 的位置，跳 1 步，然后跳 3 步到达数组的最后一个位置。");

    // 示例 2:
    let nums2 = vec![2,3,0,1,4];
    println!("\n输入: nums = {:?}", nums2);
    let result2 = Solution::jump(nums2);
    println!("输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_example_1() {
        let nums = vec![2,3,1,1,4];
        assert_eq!(Solution::jump(nums), 2);
    }

    #[test]
    fn test_jump_example_2() {
        let nums = vec![2,3,0,1,4];
        assert_eq!(Solution::jump(nums), 2);
    }

    #[test]
    fn test_jump_single_element() {
        let nums = vec![0];
        assert_eq!(Solution::jump(nums), 0);
    }

    #[test]
    fn test_jump_two_elements() {
        let nums = vec![2, 1];
        assert_eq!(Solution::jump(nums), 1);
    }

    #[test]
    fn test_jump_already_at_end() {
        let nums = vec![1];
        assert_eq!(Solution::jump(nums), 0);
    }
}