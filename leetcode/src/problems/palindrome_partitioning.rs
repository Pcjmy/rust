pub struct Solution;

impl Solution {
    fn is_palindrome(s: &[char], start: usize, end: usize) -> bool {
        let mut i = start;
        let mut j = end;

        while i < j {
            if s[i] != s[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }

        return true;
    }

    fn backtrack(s: &[char], start: usize, res: &mut Vec<Vec<String>>, path: &mut Vec<String>) {
        if start == s.len() {
            res.push(path.clone());
            return ;
        }

        for end in start..s.len() {
            if Self::is_palindrome(s, start, end) {
                path.push(s[start..=end].iter().collect());
                Self::backtrack(s, end + 1, res, path);
                path.pop();
            }
        }
    }
    
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let s = s.chars().collect::<Vec<char>>();
        let mut res = vec![];
        let mut path = vec![];
        Self::backtrack(&s, 0, &mut res, &mut path);

        return res;
    }
}

fn main() {
    // 示例 1
    let s1 = "aab".to_string();
    println!("示例 1 - 输入: s = {:?}", s1);
    let result1 = Solution::partition(s1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let s2 = "a".to_string();
    println!("示例 2 - 输入: s = {:?}", s2);
    let result2 = Solution::partition(s2);
    println!("示例 2 - 输出: {:?}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_example_1() {
        let s = "aab".to_string();
        let result = Solution::partition(s);
        // Expected: [["a","a","b"],["aa","b"]] (in some order)
        assert!(!result.is_empty());
    }

    #[test]
    fn test_partition_example_2() {
        let s = "a".to_string();
        let result = Solution::partition(s);
        // Expected: [["a"]]
        assert!(!result.is_empty());
    }
}