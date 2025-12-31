pub struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for num in nums {
            if count == 0 {
                candidate = num;
            }

            if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        return candidate;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![3, 2, 3];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::majority_element(nums1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::majority_element(nums2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_majority_element_example_1() {
        let nums = vec![3, 2, 3];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_majority_element_example_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let result = Solution::majority_element(nums);
        assert_eq!(result, 2);
    }
}