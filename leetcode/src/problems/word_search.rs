pub struct Solution;

impl Solution {
    pub fn dfs(board: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, word: &Vec<char>, i: i32, j: i32, k: usize) -> bool {
        if k == word.len() {
            return true;
        }
        if i < 0 || j < 0 || i as usize >= board.len() || j as usize >= board[0].len() || visited[i as usize][j as usize] || board[i as usize][j as usize] != word[k] {
            return false;
        }
        visited[i as usize][j as usize] = true;
        let res = Self::dfs(board, visited, word, i + 1, j, k + 1) || Self::dfs(board, visited, word, i, j + 1, k + 1) || Self::dfs(board, visited, word, i - 1, j, k + 1) || Self::dfs(board, visited, word, i, j - 1, k + 1);
        visited[i as usize][j as usize] = false;

        return res;
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word = word.chars().collect::<Vec<char>>();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::dfs(&board, &mut visited, &word, i as i32, j as i32, 0) {
                    return true;
                }
            }
        }

        return false;
    }
}

fn main() {
    // 示例 1
    let board1 = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    let word1 = "ABCCED".to_string();
    println!("示例 1 - 输入: board = {:?}, word = {:?}", board1, word1);
    let result1 = Solution::exist(board1, word1);
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let board2 = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    let word2 = "SEE".to_string();
    println!("示例 2 - 输入: board = {:?}, word = {:?}", board2, word2);
    let result2 = Solution::exist(board2, word2);
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let board3 = vec![
        vec!['A','B','C','E'],
        vec!['S','F','C','S'],
        vec!['A','D','E','E']
    ];
    let word3 = "ABCB".to_string();
    println!("示例 3 - 输入: board = {:?}, word = {:?}", board3, word3);
    let result3 = Solution::exist(board3, word3);
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_search_example_1() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let word = "ABCCED".to_string();
        let result = Solution::exist(board, word);
        assert_eq!(result, true);
    }

    #[test]
    fn test_word_search_example_2() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let word = "SEE".to_string();
        let result = Solution::exist(board, word);
        assert_eq!(result, true);
    }

    #[test]
    fn test_word_search_example_3() {
        let board = vec![
            vec!['A','B','C','E'],
            vec!['S','F','C','S'],
            vec!['A','D','E','E']
        ];
        let word = "ABCB".to_string();
        let result = Solution::exist(board, word);
        assert_eq!(result, false);
    }
}