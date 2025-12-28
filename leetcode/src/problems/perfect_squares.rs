pub struct Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut f = vec![i32::MAX; n as usize + 10];
        f[0] = 0;
        for i in 1..=n {
            let x = i * i;

            if x > n {
                break;
            }

            for j in x..=n {
                if j >= x {
                    f[j as usize] = f[j as usize].min(f[(j - x) as usize] + 1);
                }
            }
        }

        return f[n as usize];
    }
}

fn main() {
    // 示例 1
    let n1 = 12;
    println!("示例 1 - 输入: {}", n1);
    let result1 = Solution::num_squares(n1);
    println!("示例 1 - 输出: {}", result1);
    println!("解释: 12 = 4 + 4 + 4");

    // 示例 2
    let n2 = 13;
    println!("示例 2 - 输入: {}", n2);
    let result2 = Solution::num_squares(n2);
    println!("示例 2 - 输出: {}", result2);
    println!("解释: 13 = 4 + 9");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_squares_basic_example_1() {
        assert_eq!(Solution::num_squares(12), 3);
    }

    #[test]
    fn test_num_squares_basic_example_2() {
        assert_eq!(Solution::num_squares(13), 2);
    }
}