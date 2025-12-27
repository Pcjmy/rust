pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut ans = Vec::new();
        let mut vis = vec![false; len];
        let mut vec = Vec::new();

        fn dfs(x: usize, len: usize, vec: &mut Vec<i32>, vis: &mut Vec<bool>, nums: &[i32], ans: &mut Vec<Vec<i32>>) {
            if x >= len {
                ans.push(vec.clone());
                return ;
            }
            vis[x] = true;
            vec.push(nums[x]);
            dfs(x+1, len ,vec, vis, nums, ans);
            vec.pop();
            vis[x] = false;
            dfs(x+1, len, vec, vis, nums, ans);
        }

        dfs(0, len, &mut vec, &mut vis, &nums, &mut ans);

        return ans;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 2, 3];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::subsets(nums1.clone());
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let nums2 = vec![0];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::subsets(nums2.clone());
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_basic_example_1() {
        let nums = vec![1, 2, 3];
        let result = Solution::subsets(nums);
        // Expected: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
        // Note: Order may vary since subsets can be returned in any order
        assert_eq!(result.len(), 8); // 2^3 = 8 subsets
    }

    #[test]
    fn test_subsets_basic_example_2() {
        let nums = vec![0];
        let result = Solution::subsets(nums);
        // Expected: [[],[0]]
        assert_eq!(result.len(), 2); // 2^1 = 2 subsets
    }

    #[test]
    fn test_subsets_empty() {
        let nums = vec![];
        let result = Solution::subsets(nums);
        // Expected: [[]]
        assert_eq!(result.len(), 1); // 2^0 = 1 subset
        assert_eq!(result[0].len(), 0);
    }
}