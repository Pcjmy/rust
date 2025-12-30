pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n < 2 {
            return 0;
        }
        let mut min_price = prices[0];
        let mut max_profit = 0;
        for i in 1..n {
            max_profit = max_profit.max(prices[i] - min_price);
            min_price = min_price.min(prices[i]);
        }

        return max_profit;
    }
}

fn main() {
    // 示例 1
    let prices1 = vec![7, 1, 5, 3, 6, 4];
    println!("示例 1 - 输入: {:?}", prices1);
    let result1 = Solution::max_profit(prices1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let prices2 = vec![7, 6, 4, 3, 1];
    println!("示例 2 - 输入: {:?}", prices2);
    let result2 = Solution::max_profit(prices2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_profit_example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit_example_2() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit_empty() {
        let prices = vec![];
        let expected = 0;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit_single_element() {
        let prices = vec![5];
        let expected = 0;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit_increasing() {
        let prices = vec![1, 2, 3, 4, 5];
        let expected = 4;
        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_max_profit_decreasing() {
        let prices = vec![5, 4, 3, 2, 1];
        let expected = 0;
        assert_eq!(Solution::max_profit(prices), expected);
    }
}