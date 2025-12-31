pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut ans = 0;
        for i in 0..n {
            dp[i][i] = true;
            ans += 1;
        }
        for i in 0..n - 1 {
            if s[i] == s[i + 1] {
                dp[i][i + 1] = true;
                ans += 1;
            }
        }
        for i in 1..n {
            for j in 0..n - i {
                if s[j] == s[j + i] && dp[j + 1][j + i - 1] {
                    dp[j][j + i] = true;
                    ans += 1;
                }
            }
        }

        return ans;
    }
}

fn main() {
    // 示例 1
    let s1 = "abc".to_string();
    println!("palindromic_substrings - 示例 1 - 输入: {:?}", s1);
    let result1 = Solution::count_substrings(s1);
    println!("palindromic_substrings - 示例 1 - 输出: {}", result1);

    // 示例 2
    let s2 = "aaa".to_string();
    println!("palindromic_substrings - 示例 2 - 输入: {:?}", s2);
    let result2 = Solution::count_substrings(s2);
    println!("palindromic_substrings - 示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_palindromic_substrings_example_1() {
        let s = "abc".to_string();
        let expected = 3; // "a", "b", "c"
        assert_eq!(Solution::count_substrings(s), expected);
    }

    #[test]
    fn test_count_palindromic_substrings_example_2() {
        let s = "aaa".to_string();
        let expected = 6; // "a", "a", "a", "aa", "aa", "aaa"
        assert_eq!(Solution::count_substrings(s), expected);
    }
}