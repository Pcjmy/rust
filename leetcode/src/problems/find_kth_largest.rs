pub struct Solution;

impl Solution {
    pub fn find(nums: &mut Vec<i32>, l: i32, r: i32, k: i32) -> i32 {
        if l == r {
            return nums[l as usize];
        }

        let v = nums[((l + r) / 2) as usize];
        let mut x = l;
        let mut y = r;

        while x <= y {
            while nums[x as usize] > v {
                x = x + 1;
            }
            while nums[y as usize] < v {
                y = y - 1;
            }
            if x <= y {
                nums.swap(x as usize, y as usize);
                x = x + 1;
                y = y - 1;
            }
        }

        if k - 1 <= y {
            return Self::find(nums, l, y, k);
        } else if k - 1 >= x {
            return Self::find(nums, x, r, k);
        } else {
            return nums[(k - 1) as usize];
        }
    }

    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        return Self::find(&mut nums.clone(), 0, len as i32 -1, k);
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![3,2,1,5,6,4];
    let k1 = 2;
    println!("示例 1 - 输入: {:?}, k = {}", nums1, k1);
    let result1 = Solution::find_kth_largest(nums1, k1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let nums2 = vec![3,2,3,1,2,4,5,5,6];
    let k2 = 4;
    println!("示例 2 - 输入: {:?}, k = {}", nums2, k2);
    let result2 = Solution::find_kth_largest(nums2, k2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_kth_largest_example_1() {
        let nums = vec![3,2,1,5,6,4];
        let k = 2;
        let expected = 5;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_kth_largest_example_2() {
        let nums = vec![3,2,3,1,2,4,5,5,6];
        let k = 4;
        let expected = 4;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_kth_largest_single_element() {
        let nums = vec![1];
        let k = 1;
        let expected = 1;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_kth_largest_two_elements() {
        let nums = vec![2, 1];
        let k = 1;
        let expected = 2;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_kth_largest_all_same() {
        let nums = vec![3, 3, 3, 3];
        let k = 2;
        let expected = 3;
        let result = Solution::find_kth_largest(nums, k);
        assert_eq!(result, expected);
    }
}