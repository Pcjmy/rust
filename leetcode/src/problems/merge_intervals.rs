pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sorted_arr = intervals;
        let mut ans = Vec::new();
        sorted_arr.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut left = -1;
        let mut right = -1;

        for sub_arr in &sorted_arr {
            if left == -1 || right == -1 {
                left = sub_arr[0];
                right = sub_arr[1];
            } else {
                if sub_arr[0] <= right {
                    right = right.max(sub_arr[1]);
                } else {
                    ans.push(vec![left, right]);
                    left = sub_arr[0];
                    right = sub_arr[1];
                }
            }
        }

        if left != -1 && right != -1 {
            ans.push(vec![left, right]);
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let intervals1 = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
    println!("示例 1 - 输入: {:?}", intervals1);
    let result1 = Solution::merge(intervals1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let intervals2 = vec![vec![1, 4], vec![4, 5]];
    println!("示例 2 - 输入: {:?}", intervals2);
    let result2 = Solution::merge(intervals2);
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let intervals3 = vec![vec![4, 7], vec![1, 4]];
    println!("示例 3 - 输入: {:?}", intervals3);
    let result3 = Solution::merge(intervals3);
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_basic_example_1() {
        let intervals = vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]];
        let expected = vec![vec![1, 6], vec![8, 10], vec![15, 18]];
        // Solution::merge(intervals); // This will panic since unimplemented!() is called
        // Once implemented, uncomment the line above and the assertion below
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_basic_example_2() {
        let intervals = vec![vec![1, 4], vec![4, 5]];
        let expected = vec![vec![1, 5]];
        // Solution::merge(intervals); // This will panic since unimplemented!() is called
        // Once implemented, uncomment the line above and the assertion below
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_merge_basic_example_3() {
        let intervals = vec![vec![4, 7], vec![1, 4]];
        let expected = vec![vec![1, 7]];
        // Solution::merge(intervals); // This will panic since unimplemented!() is called
        // Once implemented, uncomment the line above and the assertion below
        // assert_eq!(result, expected);
    }
}