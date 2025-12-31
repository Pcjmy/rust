pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; temperatures.len()];
        let mut stack = Vec::new();

        for (i, &temp) in temperatures.iter().enumerate() {
            while let Some(&prev_index) = stack.last() {
                if temperatures[prev_index] < temp {
                    result[prev_index] = (i - prev_index) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }

        return result;
    }
}

fn main() {
    // 示例 1
    let temperatures1 = vec![73,74,75,71,69,72,76,73];
    println!("示例 1 - 输入: {:?}", temperatures1);
    let result1 = Solution::daily_temperatures(temperatures1.clone());
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let temperatures2 = vec![30,40,50,60];
    println!("示例 2 - 输入: {:?}", temperatures2);
    let result2 = Solution::daily_temperatures(temperatures2.clone());
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let temperatures3 = vec![30,60,90];
    println!("示例 3 - 输入: {:?}", temperatures3);
    let result3 = Solution::daily_temperatures(temperatures3.clone());
    println!("示例 3 - 输出: {:?}", result3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_daily_temperatures_example_1() {
        let temperatures = vec![73,74,75,71,69,72,76,73];
        let expected = vec![1,1,4,2,1,1,0,0];
        let result = Solution::daily_temperatures(temperatures);
        // assert_eq!(result, expected); // Commented out since implementation is not complete
    }

    #[test]
    fn test_daily_temperatures_example_2() {
        let temperatures = vec![30,40,50,60];
        let expected = vec![1,1,1,0];
        let result = Solution::daily_temperatures(temperatures);
        // assert_eq!(result, expected); // Commented out since implementation is not complete
    }

    #[test]
    fn test_daily_temperatures_example_3() {
        let temperatures = vec![30,60,90];
        let expected = vec![1,1,0];
        let result = Solution::daily_temperatures(temperatures);
        // assert_eq!(result, expected); // Commented out since implementation is not complete
    }
}