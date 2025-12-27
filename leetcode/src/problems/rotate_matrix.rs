pub struct Solution;

impl Solution {
    pub fn reverse(vec: &mut Vec<i32>) {
        let n = vec.len();
        let mut i = 0;
        let mut j = (n as i32 - 1) as usize;
        while i < j {
            vec.swap(i, j);
            i += 1;
            j -= 1;
        }
    }

    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        for i in 0..n {
            for j in 0..=i {
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }
        for i in 0..n {
            Self::reverse(&mut matrix[i]);
        }
    }
}

fn main() {
    // 示例 1
    let mut matrix1 = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    println!("示例 1 - 输入: {:?}", matrix1);
    Solution::rotate(&mut matrix1);
    println!("示例 1 - 输出: {:?}", matrix1);

    // 示例 2
    let mut matrix2 = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
    println!("示例 2 - 输入: {:?}", matrix2);
    Solution::rotate(&mut matrix2);
    println!("示例 2 - 输出: {:?}", matrix2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_example_1() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_rotate_matrix_example_2() {
        let mut matrix = vec![vec![5, 1, 9, 11], vec![2, 4, 8, 10], vec![13, 3, 6, 7], vec![15, 14, 12, 16]];
        let expected = vec![vec![15, 13, 2, 5], vec![14, 3, 4, 1], vec![12, 6, 8, 9], vec![16, 7, 10, 11]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_rotate_single_element() {
        let mut matrix = vec![vec![1]];
        let expected = vec![vec![1]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_rotate_two_by_two() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        let expected = vec![vec![3, 1], vec![4, 2]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, expected);
    }
}