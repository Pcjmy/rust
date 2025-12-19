pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let len = height.len();
        let mut ans = 0;
        let mut i = 0;
        let mut j = len -1;

        while i < j {
            ans = ans.max((j - i) as i32 * height[i].min(height[j]));
            if height[i] < height [j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let height1 = vec![1,8,6,2,5,4,8,3,7];
    println!("示例 1 - 输入: {:?}", height1);
    let result1 = Solution::max_area(height1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let height2 = vec![1,1];
    println!("示例 2 - 输入: {:?}", height2);
    let result2 = Solution::max_area(height2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area_basic_example_1() {
        let height = vec![1,8,6,2,5,4,8,3,7];
        let expected = 49;
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_area_basic_example_2() {
        let height = vec![1,1];
        let expected = 1;
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_area_single_pair() {
        let height = vec![2, 3];
        let expected = 2; // min(2,3) * 1 = 2
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_area_two_elements_max() {
        let height = vec![1,2];
        let expected = 1; // min(1,2) * 1 = 1
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_max_area_large_numbers() {
        let height = vec![10,9,8,7,6,5,4,3,2,1];
        let expected = 25; // 5 * 5 (indices 4 and 9: min(6,1) * (9-4) = 1*5 = 5, actually indices 1 and 9: min(9,1) * 8 = 8, or indices 1 and 2: min(9,8) * 1 = 8, or indices 0 and 9: min(10,1) * 9 = 9, or indices 0 and 1: min(10,9) * 1 = 9, or indices 4 and 5: min(6,5) * 1 = 5, optimal would be indices 0 and 5: min(10,5) * 5 = 25
        let result = Solution::max_area(height);
        assert_eq!(result, expected);
    }
}