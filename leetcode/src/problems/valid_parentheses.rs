pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        
        return stack.is_empty();
    }
}

fn main() {
    // 示例 1
    let s1 = "()".to_string();
    println!("示例 1 - 输入: {}", s1);
    println!("示例 1 - 输出: {}", Solution::is_valid(s1));

    // 示例 2
    let s2 = "()[]{}".to_string();
    println!("示例 2 - 输入: {}", s2);
    println!("示例 2 - 输出: {}", Solution::is_valid(s2));

    // 示例 3
    let s3 = "(]".to_string();
    println!("示例 3 - 输入: {}", s3);
    println!("示例 3 - 输出: {}", Solution::is_valid(s3));

    // 示例 4
    let s4 = "([])".to_string();
    println!("示例 4 - 输入: {}", s4);
    println!("示例 4 - 输出: {}", Solution::is_valid(s4));

    // 示例 5
    let s5 = "([)]".to_string();
    println!("示例 5 - 输入: {}", s5);
    println!("示例 5 - 输出: {}", Solution::is_valid(s5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_parentheses_example_1() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
    }

    #[test]
    fn test_valid_parentheses_example_2() {
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    }

    #[test]
    fn test_valid_parentheses_example_3() {
        assert_eq!(Solution::is_valid("(]".to_string()), false);
    }

    #[test]
    fn test_valid_parentheses_example_4() {
        assert_eq!(Solution::is_valid("([])".to_string()), true);
    }

    #[test]
    fn test_valid_parentheses_example_5() {
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
    }
}