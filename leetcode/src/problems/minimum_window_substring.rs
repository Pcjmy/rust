pub struct Solution;

impl Solution {
    pub fn get_char_index(c: char) -> usize {
        if c.is_uppercase() {
            c as usize - 'A' as usize
        } else if c.is_lowercase() {
            c as usize - 'a' as usize + 26
        } else {
            0
        }
    }

    pub fn get_char_count(s: &str) -> [i32; 52] {
        let mut char_count = [0; 52];
        for c in s.chars() {
            char_count[Self::get_char_index(c)] += 1;
        }
        char_count
    }

    pub fn isCovered(s_char_count: &[i32; 52], t_char_count: &[i32; 52]) -> bool {
        for i in 0..52 {
            if s_char_count[i] < t_char_count[i] {
                return false;
            }
        }
        true
    }

    pub fn min_window(s: String, t: String) -> String {
        let mut ans = String::new();
        let s_len = s.len();
        let mut left = 0;
        let mut right = 0;
        let mut min_len = usize::MAX;
        let s_chars: Vec<char> = s.chars().collect();
        let mut s_char_count = [0; 52];
        let t_char_count = Self::get_char_count(&t);
        let mut j = 0;

        for i in 0..s_len {
            if i > 0 {
                let c = s_chars[i-1];
                let c_index = Self::get_char_index(c);
                s_char_count[c_index] -= 1;
            }

            while j < s_len {
                if Self::isCovered(&s_char_count, &t_char_count) {
                    break;
                }
                let c = s_chars[j];
                let c_index = Self::get_char_index(c);
                s_char_count[c_index] += 1;
                j += 1;
            }
            
            if Self::isCovered(&s_char_count, &t_char_count) {
                if j - i < min_len {
                    min_len = j - i;
                    left = i;
                    right = j;
                }
            } else {
                break;
            }
        }

        if min_len != usize::MAX {
            ans = s[left..right].to_string();
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let s1 = "ADOBECODEBANC".to_string();
    let t1 = "ABC".to_string();
    println!("示例 1 - 输入: s = {:?}, t = {:?}", s1, t1);
    let result1 = Solution::min_window(s1, t1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let s2 = "a".to_string();
    let t2 = "a".to_string();
    println!("示例 2 - 输入: s = {:?}, t = {:?}", s2, t2);
    let result2 = Solution::min_window(s2, t2);
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let s3 = "a".to_string();
    let t3 = "aa".to_string();
    println!("示例 3 - 输入: s = {:?}, t = {:?}", s3, t3);
    let result3 = Solution::min_window(s3, t3);
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window_example_1() {
        let s = "ADOBECODEBANC".to_string();
        let t = "ABC".to_string();
        let result = Solution::min_window(s, t);
        // Expected: "BANC"
        assert_eq!(result, "BANC");
    }

    #[test]
    fn test_min_window_example_2() {
        let s = "a".to_string();
        let t = "a".to_string();
        let result = Solution::min_window(s, t);
        // Expected: "a"
        assert_eq!(result, "a");
    }

    #[test]
    fn test_min_window_example_3() {
        let s = "a".to_string();
        let t = "aa".to_string();
        let result = Solution::min_window(s, t);
        // Expected: ""
        assert_eq!(result, "");
    }
}