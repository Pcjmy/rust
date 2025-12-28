pub struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();

        for i in 0..num_rows {
            if i == 0 {
                ans.push(vec![1]);
            } else {
                let mut vec = Vec::new();
                for j in 0..=i {
                    if j == 0 {
                        vec.push(ans[(i-1) as usize][j as usize]);
                    } else if j < i {
                        vec.push(ans[(i-1) as usize][(j-1) as usize] + ans[(i-1) as usize][j as usize]);
                    } else {
                        vec.push(ans[(i-1) as usize][(j-1) as usize]);
                    }
                }
                ans.push(vec);
            }
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let result1 = Solution::generate(5);
    println!("Triangle 示例 1 - 输入: numRows = 5");
    println!("Triangle 示例 1 - 输出: {:?}", result1);

    // 示例 2
    let result2 = Solution::generate(1);
    println!("Triangle 示例 2 - 输入: numRows = 1");
    println!("Triangle 示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_basic_example_1() {
        let result = Solution::generate(5);
        let expected = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1]
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_triangle_basic_example_2() {
        let result = Solution::generate(1);
        let expected = vec![vec![1]];
        assert_eq!(result, expected);
    }
}