pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut vec = nums.clone();
        vec.sort();
        let len = vec.len();

        for i in 0..len {
            if i > 0 && vec[i] == vec[i-1] {
                continue;
            }

            let mut k = len - 1;

            for j in i+1..len {
                if j >= k {
                    break;
                }

                if j > i + 1 && vec[j] == vec[j-1] {
                    continue;
                }

                while j + 1 < k && vec[i] + vec[j] + vec[k] > 0 {
                    k -= 1;
                }

                let sum = vec[i] + vec[j] + vec[k];

                if sum == 0 {
                    ans.push(vec![vec[i], vec[j], vec[k]]);
                }
            }
        }
        return ans;
    }
}

fn main() {
    // 示例 1
    let nums1 = vec![-1, 0, 1, 2, -1, -4];
    println!("示例 1 - 输入: {:?}", nums1);
    let result1 = Solution::three_sum(nums1.clone());
    println!("示例 1 - 输出: {:?}", result1);

    // 示例 2
    let nums2 = vec![0, 1, 1];
    println!("示例 2 - 输入: {:?}", nums2);
    let result2 = Solution::three_sum(nums2.clone());
    println!("示例 2 - 输出: {:?}", result2);

    // 示例 3
    let nums3 = vec![0, 0, 0];
    println!("示例 3 - 输入: {:?}", nums3);
    let result3 = Solution::three_sum(nums3.clone());
    println!("示例 3 - 输出: {:?}", result3);
}
