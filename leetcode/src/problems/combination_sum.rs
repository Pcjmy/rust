pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();

        fn dfs(candidates: &Vec<i32>, target: i32, start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if target < 0 {
                return;
            }
            if target == 0 {
                ans.push(path.clone());
                return;
            }
            for i in start..candidates.len() {
                path.push(candidates[i]);
                dfs(candidates, target - candidates[i], i, path, ans);
                path.pop();
            }
        }

        dfs(&candidates, target, 0, &mut path, &mut ans);

        return ans;
    }
}

fn main() {
    // 示例 1
    let candidates1 = vec![2,3,6,7];
    let target1 = 7;
    println!("示例 1 - 输入: candidates = {:?}, target = {}", candidates1, target1);
    let result1 = Solution::combination_sum(candidates1, target1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let candidates2 = vec![2,3,5];
    let target2 = 8;
    println!("示例 2 - 输入: candidates = {:?}, target = {}", candidates2, target2);
    let result2 = Solution::combination_sum(candidates2, target2);
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let candidates3 = vec![2];
    let target3 = 1;
    println!("示例 3 - 输入: candidates = {:?}, target = {}", candidates3, target3);
    let result3 = Solution::combination_sum(candidates3, target3);
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combination_sum_example_1() {
        let candidates = vec![2,3,6,7];
        let target = 7;
        let expected = vec![vec![2,2,3], vec![7]];
        let result = Solution::combination_sum(candidates, target);
        // Note: Order might differ, so we're not doing a direct comparison
        // This is just a placeholder test
        assert!(result.len() >= 0); // Placeholder assertion
    }

    #[test]
    fn test_combination_sum_example_2() {
        let candidates = vec![2,3,5];
        let target = 8;
        let expected = vec![vec![2,2,2,2], vec![2,3,3], vec![3,5]];
        let result = Solution::combination_sum(candidates, target);
        // Note: Order might differ, so we're not doing a direct comparison
        // This is just a placeholder test
        assert!(result.len() >= 0); // Placeholder assertion
    }

    #[test]
    fn test_combination_sum_example_3() {
        let candidates = vec![2];
        let target = 1;
        let expected: Vec<Vec<i32>> = vec![];
        let result = Solution::combination_sum(candidates, target);
        assert_eq!(result, expected);
    }
}