pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut low = 0;
        let mut mid = 0;
        let mut high = nums.len() as i32 - 1;

        while mid <= high {
            match nums[mid as usize] {
                0 => {
                    nums.swap(low as usize, mid as usize);
                    low += 1;
                    mid += 1;
                }
                1 => {
                    mid += 1;
                }
                2 => {
                    nums.swap(mid as usize, high as usize);
                    high -= 1;
                }
                _ => {
                    mid += 1;
                }
            }
        }
    }
}

fn main() {
    // 示例 1
    let mut nums1 = vec![2, 0, 2, 1, 1, 0];
    println!("示例 1 - 输入: {:?}", nums1);
    Solution::sort_colors(&mut nums1);
    println!("示例 1 - 输出: {:?}", nums1);

    // 示例 2
    let mut nums2 = vec![2, 0, 1];
    println!("示例 2 - 输入: {:?}", nums2);
    Solution::sort_colors(&mut nums2);
    println!("示例 2 - 输出: {:?}", nums2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_colors_basic_example_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_basic_example_2() {
        let mut nums = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_single_element() {
        let mut nums = vec![1];
        let expected = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_all_same_color_0() {
        let mut nums = vec![0, 0, 0];
        let expected = vec![0, 0, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_all_same_color_1() {
        let mut nums = vec![1, 1, 1];
        let expected = vec![1, 1, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_all_same_color_2() {
        let mut nums = vec![2, 2, 2];
        let expected = vec![2, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_already_sorted() {
        let mut nums = vec![0, 0, 1, 1, 2, 2];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }

    #[test]
    fn test_sort_colors_reverse_sorted() {
        let mut nums = vec![2, 2, 1, 1, 0, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, expected);
    }
}