struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        use std::collections::HashSet;
        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut max_length = 0;

        for &num in num_set.iter() {
            if !num_set.contains(&(num - 1)) {
                let mut current_num = num;
                let mut current_length = 1;

                while num_set.contains(&(current_num + 1)) {
                    current_num += 1;
                    current_length += 1;
                }

                max_length = max_length.max(current_length);
            }
        }

        max_length
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![100, 4, 200, 1, 3, 2];
    let result1 = Solution::longest_consecutive(nums1);
    println!("示例 1 输出: {}", result1); // 期望结果: 4

    // 示例 2
    let nums2 = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
    let result2 = Solution::longest_consecutive(nums2);
    println!("示例 2 输出: {}", result2); // 期望结果: 9

    // 示例 3
    let nums3 = vec![1, 0, 1, 2];
    let result3 = Solution::longest_consecutive(nums3);
    println!("示例 3 输出: {}", result3); // 期望结果: 3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
        assert_eq!(Solution::longest_consecutive(vec![1, 0, 1, 2]), 3);
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
        assert_eq!(Solution::longest_consecutive(vec![1]), 1);
        assert_eq!(Solution::longest_consecutive(vec![1, 2, 3, 4, 5]), 5);
    }
}