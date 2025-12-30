pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let n = s.len();
        let mut f = vec![false; n + 1];
        f[0] = true;
        for i in 1..=n {
            for j in 0..i {
                if f[j] && word_dict.contains(&s[j..i].to_string()) {
                    f[i] = true;
                    break;
                }
            }
        }
        return f[n];
    }
}

fn main() {
    // 示例 1
    let s1 = String::from("leetcode");
    let word_dict1 = vec![String::from("leet"), String::from("code")];
    println!("示例 1 - 输入: s = {:?}, wordDict = {:?}", s1, word_dict1);
    let result1 = Solution::word_break(s1, word_dict1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let s2 = String::from("applepenapple");
    let word_dict2 = vec![String::from("apple"), String::from("pen")];
    println!("示例 2 - 输入: s = {:?}, wordDict = {:?}", s2, word_dict2);
    let result2 = Solution::word_break(s2, word_dict2);
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let s3 = String::from("catsandog");
    let word_dict3 = vec![String::from("cats"), String::from("dog"), String::from("sand"), String::from("and"), String::from("cat")];
    println!("示例 3 - 输入: s = {:?}, wordDict = {:?}", s3, word_dict3);
    let result3 = Solution::word_break(s3, word_dict3);
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_break_basic_example_1() {
        let s = String::from("leetcode");
        let word_dict = vec![String::from("leet"), String::from("code")];
        let expected = true;
        let result = Solution::word_break(s, word_dict);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_word_break_basic_example_2() {
        let s = String::from("applepenapple");
        let word_dict = vec![String::from("apple"), String::from("pen")];
        let expected = true;
        let result = Solution::word_break(s, word_dict);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_word_break_basic_example_3() {
        let s = String::from("catsandog");
        let word_dict = vec![String::from("cats"), String::from("dog"), String::from("sand"), String::from("and"), String::from("cat")];
        let expected = false;
        let result = Solution::word_break(s, word_dict);
        assert_eq!(result, expected);
    }
}