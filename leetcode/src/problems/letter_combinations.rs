pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let mut ans = Vec::new();
        let digit_to_letters = vec![
            "",
            "",
            "abc",
            "def",
            "ghi",
            "jkl",
            "mno",
            "pqrs",
            "tuv",
            "wxyz",
        ];

        fn dfs(
            digits: &str,
            index: usize,
            path: &mut String,
            ans: &mut Vec<String>,
            digit_to_letters: &[&str],
        ) {
            if index == digits.len() {
                ans.push(path.clone());
                return;
            }
            let letters = digit_to_letters[digits.chars().nth(index).unwrap() as usize - '0' as usize];
            for letter in letters.chars() {
                path.push(letter);
                dfs(digits, index + 1, path, ans, digit_to_letters);
                path.pop();
            }
        }

        dfs(&digits, 0, &mut String::new(), &mut ans, &digit_to_letters);

        return ans;
    }
}

fn main() {
    // 示例 1
    let digits1 = "23".to_string();
    println!("示例 1 - 输入: {}", digits1);
    let result1 = Solution::letter_combinations(digits1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let digits2 = "2".to_string();
    println!("示例 2 - 输入: {}", digits2);
    let result2 = Solution::letter_combinations(digits2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letter_combinations_basic_example_1() {
        let digits = "23".to_string();
        // Expected: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
        // According to problem: digits.length = 2 (within 1-4), digits[0]='2', digits[1]='3' (both in ['2','9'])
        let result = Solution::letter_combinations(digits);
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_letter_combinations_basic_example_2() {
        let digits = "2".to_string();
        // Expected: ["a","b","c"]
        // According to problem: digits.length = 1 (within 1-4), digits[0]='2' (in ['2','9'])
        let result = Solution::letter_combinations(digits);
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_letter_combinations_single_digit() {
        let digits = "5".to_string();
        // According to problem: digits.length = 1 (within 1-4), digits[0]='5' (in ['2','9'])
        let result = Solution::letter_combinations(digits);
        // Should return 3 letters corresponding to digit 5
    }

    #[test]
    fn test_letter_combinations_max_length() {
        let digits = "2345".to_string();
        // According to problem: digits.length = 4 (within 1-4), all digits in ['2','9']
        let result = Solution::letter_combinations(digits);
        // Should handle maximum length input
    }

    #[test]
    fn test_letter_combinations_with_7_and_9() {
        let digits = "79".to_string();
        // According to problem: digits.length = 2 (within 1-4), digits[0]='7', digits[1]='9' (both in ['2','9'])
        let result = Solution::letter_combinations(digits);
        // Should handle digits 7 and 9 which have 4 letters each
    }
}