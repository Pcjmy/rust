pub struct Solution;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut last = [0; 26];
        for (i, c) in s.chars().enumerate() {
            last[c as usize - 'a' as usize] = i;
        }

        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (i, c) in s.chars().enumerate() {
            end = end.max(last[c as usize - 'a' as usize]);
            if i == end {
                result.push((end - start + 1) as i32);
                start = end + 1;
            }
        }

        return result;
    }
}

fn main() {
    // 示例 1
    let s1 = "ababcbacadefegdehijhklij".to_string();
    println!("示例 1 - 输入: {}", s1);
    let result1 = Solution::partition_labels(s1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let s2 = "eccbbbbdec".to_string();
    println!("示例 2 - 输入: {}", s2);
    let result2 = Solution::partition_labels(s2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_labels_example_1() {
        let s = "ababcbacadefegdehijhklij".to_string();
        let expected = vec![9, 7, 8];
        let result = Solution::partition_labels(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_partition_labels_example_2() {
        let s = "eccbbbbdec".to_string();
        let expected = vec![10];
        let result = Solution::partition_labels(s);
        assert_eq!(result, expected);
    }
}