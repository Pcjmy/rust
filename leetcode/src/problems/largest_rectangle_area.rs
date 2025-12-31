pub struct Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut stack1 = Vec::new();
        let mut stack2 = Vec::new();
        let mut left = vec![0; heights.len()];
        let mut max_area = 0;

        for (i, &h) in heights.iter().enumerate() {
            while let Some(&prev_index) = stack1.last() {
                if heights[prev_index] >= h {
                    stack1.pop();
                } else {
                    break;
                }
            }

            if let Some(&prev_index) = stack1.last() {
                left[i] = prev_index as i32;
            } else {
                left[i] = -1;
            }

            stack1.push(i);
        }

        for (i, &h) in heights.iter().enumerate().rev() {
            while let Some(&prev_index) = stack2.last() {
                if heights[prev_index] >= h {
                    stack2.pop();
                } else {
                    break;
                }
            }

            if let Some(&prev_index) = stack2.last() {
                let mut width;
                if left[i] == -1 {
                    width = prev_index as i32;
                } else {
                    width = prev_index as i32 - left[i] - 1;
                }
                let area = width * h;
                max_area = max_area.max(area);
            } else {
                let mut width;
                if left[i] == -1 {
                    width = heights.len() as i32;
                } else {
                    width = heights.len() as i32 - left[i] - 1;
                }
                let area = width * h;
                max_area = max_area.max(area);
            }

            stack2.push(i);
        }


        return max_area;
    }
}

fn main() {
    // 示例 1
    let heights1 = vec![2,1,5,6,2,3];
    println!("示例 1 - 输入: {:?}", heights1);
    let result1 = Solution::largest_rectangle_area(heights1);
    println!("示例 1 - 输出: {}", result1);

    // 示例 2
    let heights2 = vec![2,4];
    println!("示例 2 - 输入: {:?}", heights2);
    let result2 = Solution::largest_rectangle_area(heights2);
    println!("示例 2 - 输出: {}", result2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_rectangle_area_example_1() {
        let heights = vec![2,1,5,6,2,3];
        let expected = 10;
        let result = Solution::largest_rectangle_area(heights);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_largest_rectangle_area_example_2() {
        let heights = vec![2,4];
        let expected = 4;
        let result = Solution::largest_rectangle_area(heights);
        assert_eq!(result, expected);
    }
}