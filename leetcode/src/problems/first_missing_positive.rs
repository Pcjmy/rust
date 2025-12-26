pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut arr = nums;
        let len = arr.len();
        for i in 0..len {
            if arr[i] <= 0 || arr[i] > len as i32 {
                arr[i] = len as i32 + 1;
            }
        }
        for i in 0..len {
            let x = arr[i].abs();
            if x > 0 && x <= len as i32 && arr[(x - 1) as usize] > 0 {
                arr[(x - 1) as usize] = -arr[(x - 1) as usize];
            }
        }
        for i in 0..len {
            if arr[i] >= 0 {
                return i as i32 + 1;
            }
        }
        return len as i32 + 1;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![1, 2, 0];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::first_missing_positive(nums1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![3, 4, -1, 1];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::first_missing_positive(nums2);
    println!("示例 2 - 输出: {}", result2);

    // 示例 3
    let nums3 = vec![7, 8, 9, 11, 12];
    println!("示例 3 - 输入: {:?}", nums3);
    let result3 = Solution::first_missing_positive(nums3);
    println!("示例 3 - 输出: {}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_missing_positive_example_1() {
        let nums = vec![1, 2, 0];
        let expected = 3;
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }

    #[test]
    fn test_first_missing_positive_example_2() {
        let nums = vec![3, 4, -1, 1];
        let expected = 2;
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }

    #[test]
    fn test_first_missing_positive_example_3() {
        let nums = vec![7, 8, 9, 11, 12];
        let expected = 1;
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }

    #[test]
    fn test_first_missing_positive_empty() {
        let nums = vec![];
        let expected = 1;
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }

    #[test]
    fn test_first_missing_positive_all_negative() {
        let nums = vec![-1, -2, -3];
        let expected = 1;
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }

    #[test]
    fn test_first_missing_positive_consecutive_starting_from_1() {
        let nums = vec![1, 2, 3, 4, 5];
        let expected = 6;
        assert_eq!(Solution::first_missing_positive(nums), expected);
    }
}