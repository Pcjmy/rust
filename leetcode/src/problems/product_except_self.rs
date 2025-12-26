pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut product = 1;
        let mut cnt = 0;
        let len = nums.len();
        for i in 0..len {
            if nums[i] == 0 {
                cnt += 1;
            } else {
                product *= nums[i];
            }
        }
        for i in 0..len {
            if cnt >= 2 {
                ans.push(0);
            } else if cnt == 1 {
                if nums[i] == 0 {
                    ans.push(product);
                } else {
                    ans.push(0);
                }
            } else {
                ans.push(product / nums[i]);
            }
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 2, 3, 4];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::product_except_self(nums1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let nums2 = vec![-1, 1, 0, -3, 3];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::product_except_self(nums2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_except_self_example_1() {
        let nums = vec![1, 2, 3, 4];
        let expected = vec![24, 12, 8, 6];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_product_except_self_example_2() {
        let nums = vec![-1, 1, 0, -3, 3];
        let expected = vec![0, 0, 9, 0, 0];
        let result = Solution::product_except_self(nums);
        assert_eq!(result, expected);
    }
}