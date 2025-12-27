pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut ans = Vec::new();
        let mut vec = vec![0; len];
        let mut vis = vec![false; len];

        fn dfs(x: usize, nums: &[i32], len: usize, ans: &mut Vec<Vec<i32>>, vec: &mut Vec<i32>, vis: &mut Vec<bool>) {
            if x >= len {
                ans.push(vec.clone());
                return ;
            }
            for i in 0..len {
                if !vis[i] {
                    vec[x] = nums[i];
                    vis[i] = true;
                    dfs(x+1, nums, len, ans, vec, vis);
                    vis[i] = false;
                }
            }
        }

        dfs(0, &nums, len, &mut ans, &mut vec, &mut vis);

        return ans;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 2, 3];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::permute(nums1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let nums2 = vec![0, 1];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::permute(nums2);
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let nums3 = vec![1];
    println!("示例 3 - 输入: {:?}", nums3);
    let result3 = Solution::permute(nums3);
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute_basic_example_1() {
        let nums = vec![1, 2, 3];
        let result = Solution::permute(nums);
        // Expected: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
        // Note: The order of permutations might vary
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_permute_basic_example_2() {
        let nums = vec![0, 1];
        let result = Solution::permute(nums);
        // Expected: [[0,1],[1,0]]
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_permute_basic_example_3() {
        let nums = vec![1];
        let result = Solution::permute(nums);
        // Expected: [[1]]
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_permute_empty() {
        let nums = vec![];
        let result = Solution::permute(nums);
        assert_eq!(result.len(), 1); // Should return [[]]
    }
}