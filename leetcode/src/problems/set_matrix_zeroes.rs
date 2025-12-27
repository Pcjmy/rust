pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut row = vec![false; n];
        let mut col = vec![false; m];

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    row[i] = true;
                    col[j] = true;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if row[i] || col[j] {
                    matrix[i][j] = 0;
                }
            } 
        }
    }
}

fn main() {
    // 示例 1
    let mut matrix1 = vec![vec![1,1,1], vec![1,0,1], vec![1,1,1]];
    println!("示例 1 - 输入: {:?}", matrix1);
    Solution::set_zeroes(&mut matrix1);
    println!("示例 1 - 输出: {:?}", matrix1);

    // 示例 2
    let mut matrix2 = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
    println!("示例 2 - 输入: {:?}", matrix2);
    Solution::set_zeroes(&mut matrix2);
    println!("示例 2 - 输出: {:?}", matrix2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_zeroes_example_1() {
        let mut matrix = vec![vec![1,1,1], vec![1,0,1], vec![1,1,1]];
        let expected = vec![vec![1,0,1], vec![0,0,0], vec![1,0,1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_set_zeroes_example_2() {
        let mut matrix = vec![vec![0,1,2,0], vec![3,4,5,2], vec![1,3,1,5]];
        let expected = vec![vec![0,0,0,0], vec![0,4,5,0], vec![0,3,1,0]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, expected);
    }
}