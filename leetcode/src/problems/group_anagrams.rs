use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort_unstable();
            let key = chars.into_iter().collect::<String>();
            map.entry(key).or_default().push(s);
        }

        map.into_values().collect()
    }
}

fn main() {
    // 示例 1
    let strs1 = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let result1 = Solution::group_anagrams(strs1);
    println!("示例 1 结果: {:?}", result1);

    // 示例 2
    let strs2 = vec!["".to_string()];
    let result2 = Solution::group_anagrams(strs2);
    println!("示例 2 结果: {:?}", result2);

    // 示例 3
    let strs3 = vec!["a".to_string()];
    let result3 = Solution::group_anagrams(strs3);
    println!("示例 3 结果: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let strs1 = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        // 测试用例将在实现解决方案后使用
        // 预期输出: [["bat"],["nat","tan"],["ate","eat","tea"]]

        let strs2 = vec!["".to_string()];
        // 预期输出: [[""]]

        let strs3 = vec!["a".to_string()];
        // 预期输出: [["a"]]
    }
}