use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            
            if let Some(&index) = map.get(&complement) {
                return vec![index as i32, i as i32];
            }
            
            map.insert(num, i);
        }
        
        vec![] // 根据题目约束，不应该到达这里
    }
}

fn main() {
    // 示例用法
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("索引: {:?}", result); // 期望结果: [0, 1]
    
    // 另一个测试案例
    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    let result2 = Solution::two_sum(nums2, target2);
    println!("索引: {:?}", result2); // 期望结果: [1, 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}