pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let n = s.len();
        let mut f = vec![0; n];
        let s_chars: Vec<char> = s.chars().collect();
        for i in 1..n {
            if s_chars[i] == ')' {
                if s_chars[(i - 1) as usize] == '(' {
                    if i >= 2 {
                        f[i] = f[(i - 2) as usize] + 2;
                    } else {
                        f[i] = 2;
                    }
                } else if i >= (f[(i - 1) as usize] + 1) as usize && s_chars[(i - f[(i - 1) as usize] - 1) as usize] == '(' {
                    if i >= f[(i - 1) as usize] + 2 {
                        f[i] = f[(i - f[(i - 1) as usize] - 2) as usize] + 2 + f[(i - 1) as usize];
                    } else {
                        f[i] = 2 + f[(i - 1) as usize];
                    }
                }
            }
        }

        return *f.iter().max().unwrap_or(&0) as i32;
    }
}

fn main() {
    // 示例 1
    let s1 = String::from("(()");
    println!("示例 1 - 输入: {:?}", s1);
    let result1 = Solution::longest_valid_parentheses(s1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let s2 = String::from(")()())");
    println!("示例 2 - 输入: {:?}", s2);
    let result2 = Solution::longest_valid_parentheses(s2);
    println!("示例 2 - 输出: {}", result2);

    // 示例 3
    let s3 = String::from("");
    println!("示例 3 - 输入: {:?}", s3);
    let result3 = Solution::longest_valid_parentheses(s3);
    println!("示例 3 - 输出: {}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_valid_parentheses_example_1() {
        let s = String::from("(()");
        let expected = 2;
        let result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_longest_valid_parentheses_example_2() {
        let s = String::from(")()())");
        let expected = 4;
        let result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_longest_valid_parentheses_example_3() {
        let s = String::from("");
        let expected = 0;
        let result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_longest_valid_parentheses_all_valid() {
        let s = String::from("()()");
        let expected = 4;
        let result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_longest_valid_parentheses_complex() {
        let s = String::from("(()())");
        let expected = 6;
        let result = Solution::longest_valid_parentheses(s);
        assert_eq!(result, expected);
    }
}