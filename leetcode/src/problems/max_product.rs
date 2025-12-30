pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut f = vec![0; n];
        let mut g = vec![0; n];
        f[0] = nums[0];
        g[0] = nums[0];
        for i in 1..n {
            f[i] = nums[i].max(f[i - 1] * nums[i]).max(g[i - 1] * nums[i]);
            g[i] = nums[i].min(f[i - 1] * nums[i]).min(g[i - 1] * nums[i]);
        }
        return *f.iter().max().unwrap();
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![2, 3, -2, 4];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::max_product(nums1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![-2, 0, -1];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::max_product(nums2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_basic_example_1() {
        let nums = vec![2, 3, -2, 4];
        let expected = 6;
        assert_eq!(Solution::max_product(nums), expected);
    }

    #[test]
    fn test_max_product_basic_example_2() {
        let nums = vec![-2, 0, -1];
        let expected = 0;
        assert_eq!(Solution::max_product(nums), expected);
    }
}