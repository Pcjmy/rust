pub struct Solution;

impl Solution {
    pub fn get_char_count(s: &str) -> [i32; 26] {
        let mut count = [0; 26];
        for c in s.chars() {
            count[(c as usize) - ('a' as usize)] += 1;
        }
        return count;
    }

    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let s_len = s.len();
        let p_len = p.len();
        if s_len < p_len {
            return vec![];
        }

        let mut s_count = Self::get_char_count(&s[0..p_len]);
        let p_count = Self::get_char_count(&p);

        let mut ans = Vec::new();
        for i in 0..=s_len - p_len {
            if i > 0 {
                s_count[(s.as_bytes()[i-1] - b'a') as usize] -= 1;
                s_count[(s.as_bytes()[i+p_len-1] - b'a') as usize] += 1;
            }

            if s_count == p_count {
                ans.push(i as i32);
            }
        }
        return ans;
    }
}

fn main() {
    // 示例 1
    let s1 = "cbaebabacd".to_string();
    let p1 = "abc".to_string();
    println!("示例 1 - 输入: s = {:?}, p = {:?}", s1, p1);
    let result1 = Solution::find_anagrams(s1, p1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let s2 = "abab".to_string();
    let p2 = "ab".to_string();
    println!("示例 2 - 输入: s = {:?}, p = {:?}", s2, p2);
    let result2 = Solution::find_anagrams(s2, p2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_anagrams_example_1() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        let expected = vec![0, 6];
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_anagrams_example_2() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        let expected = vec![0, 1, 2];
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_anagrams_empty_string() {
        let s = "".to_string();
        let p = "ab".to_string();
        let expected = vec![];
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_find_anagrams_p_longer_than_s() {
        let s = "a".to_string();
        let p = "ab".to_string();
        let expected = vec![];
        let result = Solution::find_anagrams(s, p);
        assert_eq!(result, expected);
    }
}