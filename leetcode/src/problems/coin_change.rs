pub struct Solution;

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        use std::cmp::min;

        let mut f = vec![i32::MAX; amount as usize + 10];
        f[0] = 0;
        let len = coins.len();
        for i in 0..len {
            let x = coins[i];
            if x > amount {
                continue;
            }
            for j in x..=amount {
                if f[(j - x) as usize] != i32::MAX {
                    f[j as usize] = min(f[j as usize], f[(j - x) as usize] + 1);
                }
            }
        }

        if f[amount as usize] == i32::MAX {
            return -1;
        }

        return f[amount as usize];
    }
}

fn main() {
    // 示例 1
    let coins1 = vec![1, 2, 5];
    let amount1 = 11;
    println!("示例 1 - 输入: coins: {:?}, amount: {}", coins1, amount1);
    let result1 = Solution::coin_change(coins1, amount1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let coins2 = vec![2];
    let amount2 = 3;
    println!("示例 2 - 输入: coins: {:?}, amount: {}", coins2, amount2);
    let result2 = Solution::coin_change(coins2, amount2);
    println!("示例 2 - 输出: {}", result2);

    // 示例 3
    let coins3 = vec![1];
    let amount3 = 0;
    println!("示例 3 - 输入: coins: {:?}, amount: {}", coins3, amount3);
    let result3 = Solution::coin_change(coins3, amount3);
    println!("示例 3 - 输出: {}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change_example_1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let expected = 3; // 11 = 5 + 5 + 1
        assert_eq!(Solution::coin_change(coins, amount), expected);
    }

    #[test]
    fn test_coin_change_example_2() {
        let coins = vec![2];
        let amount = 3;
        let expected = -1; // Cannot make 3 with only coin of value 2
        assert_eq!(Solution::coin_change(coins, amount), expected);
    }

    #[test]
    fn test_coin_change_example_3() {
        let coins = vec![1];
        let amount = 0;
        let expected = 0; // 0 coins needed to make amount 0
        assert_eq!(Solution::coin_change(coins, amount), expected);
    }

    #[test]
    fn test_coin_change_no_solution() {
        let coins = vec![2];
        let amount = 1;
        let expected = -1; // Cannot make 1 with only coin of value 2
        assert_eq!(Solution::coin_change(coins, amount), expected);
    }

    #[test]
    fn test_coin_change_single_coin() {
        let coins = vec![1];
        let amount = 5;
        let expected = 5; // 5 coins of value 1
        assert_eq!(Solution::coin_change(coins, amount), expected);
    }
}