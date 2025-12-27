pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut vis: Vec<Vec<bool>> = vec![vec![false; m]; n];
        let dir: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut i = 0;
        let mut cnt = 0;
        let mut ans = Vec::new();
        let mut x = 0;
        let mut y = 0;
        vis[0][0] = true;
        ans.push(matrix[0][0]);
        cnt += 1;

        while cnt < n * m {
            let (dx, dy) = dir[i];
            while true {
                let _x = x + dx;
                let _y = y + dy;
                if _x >= 0 && _x < n as i32 && _y >= 0 && _y < m as i32 && !vis[_x as usize][_y as usize] {
                    vis[_x as usize][_y as usize] = true;
                    ans.push(matrix[_x as usize][_y as usize]);
                    cnt += 1;
                    x = _x;
                    y = _y;
                } else {
                    break;
                }
            }
            i = (i + 1) % 4;
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let matrix1 = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
    println!("示例 1 - 输入: {:?}", matrix1);
    let result1 = Solution::spiral_order(matrix1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let matrix2 = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
    println!("示例 2 - 输入: {:?}", matrix2);
    let result2 = Solution::spiral_order(matrix2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_order_basic_example_1() {
        let matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let expected = vec![1,2,3,6,9,8,7,4,5];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spiral_order_basic_example_2() {
        let matrix = vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]];
        let expected = vec![1,2,3,4,8,12,11,10,9,5,6,7];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spiral_order_single_row() {
        let matrix = vec![vec![1,2,3,4]];
        let expected = vec![1,2,3,4];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spiral_order_single_column() {
        let matrix = vec![vec![1],vec![2],vec![3],vec![4]];
        let expected = vec![1,2,3,4];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spiral_order_single_element() {
        let matrix = vec![vec![1]];
        let expected = vec![1];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_spiral_order_two_by_two() {
        let matrix = vec![vec![1,2],vec![3,4]];
        let expected = vec![1,2,4,3];
        let result = Solution::spiral_order(matrix);
        assert_eq!(result, expected);
    }
}