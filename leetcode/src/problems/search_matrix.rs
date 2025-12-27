pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut x = 0;
        let mut y = m as i32 - 1;

        while x < n && y >= 0 {
            if matrix[x as usize][y as usize] == target {
                return true;
            } else if matrix[x as usize][y as usize] < target {
                x += 1;
            } else {
                y -= 1;
            }
        }

        return false;
    }
}

fn main() {
    // 示例 1
    let matrix1 = vec![
        vec![1,4,7,11,15],
        vec![2,5,8,12,19],
        vec![3,6,9,16,22],
        vec![10,13,14,17,24],
        vec![18,21,23,26,30]
    ];
    let target1 = 5;
    println!("示例 1 - 输入: matrix = {:?}, target = {}", matrix1, target1);
    let result1 = Solution::search_matrix(matrix1, target1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let matrix2 = vec![
        vec![1,4,7,11,15],
        vec![2,5,8,12,19],
        vec![3,6,9,16,22],
        vec![10,13,14,17,24],
        vec![18,21,23,26,30]
    ];
    let target2 = 20;
    println!("示例 2 - 输入: matrix = {:?}, target = {}", matrix2, target2);
    let result2 = Solution::search_matrix(matrix2, target2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_matrix_example_1() {
        let matrix = vec![
            vec![1,4,7,11,15],
            vec![2,5,8,12,19],
            vec![3,6,9,16,22],
            vec![10,13,14,17,24],
            vec![18,21,23,26,30]
        ];
        let target = 5;
        let result = Solution::search_matrix(matrix, target);
        assert_eq!(result, true);
    }

    #[test]
    fn test_search_matrix_example_2() {
        let matrix = vec![
            vec![1,4,7,11,15],
            vec![2,5,8,12,19],
            vec![3,6,9,16,22],
            vec![10,13,14,17,24],
            vec![18,21,23,26,30]
        ];
        let target = 20;
        let result = Solution::search_matrix(matrix, target);
        assert_eq!(result, false);
    }
}