pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack = Vec::new();
        let mut current_num = 0;
        let mut current_string = String::new();

        for c in s.chars() {
            if c.is_digit(10) {
                current_num = current_num * 10 + c.to_digit(10).unwrap() as i32;
            } else if c == '[' {
                stack.push(current_string);
                stack.push(current_num.to_string());
                current_string = String::new();
                current_num = 0;
            } else if c == ']' {
                let num: i32 = stack.pop().unwrap().parse().unwrap();
                let prev_string = stack.pop().unwrap();
                current_string = prev_string + &current_string.repeat(num as usize);
            } else {
                current_string.push(c);
            }
        }

        return current_string;
    }
}

fn main() {
    // 示例 1
    let s1 = "3[a]2[bc]".to_string();
    println!("示例 1 - 输入: {}", s1);
    let result1 = Solution::decode_string(s1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let s2 = "3[a2[c]]".to_string();
    println!("示例 2 - 输入: {}", s2);
    let result2 = Solution::decode_string(s2);
    println!("示例 2 - 输出: {}", result2);

    // 示例 3
    let s3 = "2[abc]3[cd]ef".to_string();
    println!("示例 3 - 输入: {}", s3);
    let result3 = Solution::decode_string(s3);
    println!("示例 3 - 输出: {}", result3);

    // 示例 4
    let s4 = "abc3[cd]xyz".to_string();
    println!("示例 4 - 输入: {}", s4);
    let result4 = Solution::decode_string(s4);
    println!("示例 4 - 输出: {}", result4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_string_example_1() {
        let s = "3[a]2[bc]".to_string();
        let expected = "aaabcbc".to_string();
        assert_eq!(Solution::decode_string(s), expected);
    }

    #[test]
    fn test_decode_string_example_2() {
        let s = "3[a2[c]]".to_string();
        let expected = "accaccacc".to_string();
        assert_eq!(Solution::decode_string(s), expected);
    }

    #[test]
    fn test_decode_string_example_3() {
        let s = "2[abc]3[cd]ef".to_string();
        let expected = "abcabccdcdcdef".to_string();
        assert_eq!(Solution::decode_string(s), expected);
    }

    #[test]
    fn test_decode_string_example_4() {
        let s = "abc3[cd]xyz".to_string();
        let expected = "abccdcdcdxyz".to_string();
        assert_eq!(Solution::decode_string(s), expected);
    }
}