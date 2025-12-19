pub struct Solution;

impl Solution {
    pub fn normalize_triplet(mut triplet: [i32; 3]) -> [i32; 3] {
        triplet.sort();
        triplet
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        use std::collections::HashMap;

        let len = nums.len();
        let num_count: HashMap<i32, usize> = nums.iter().cloned().fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
        let mut ans_set: HashSet<[i32; 3]> = HashSet::new();

        for i in 0..len {
            for j in i+1..len {
                let target = -(nums[i] + nums[j]);
                let mut cnt = 1;
                if nums[i] == target {
                    cnt += 1;
                }
                if nums[j] == target {
                    cnt += 1;
                }
                if let Some(&cnt2) = num_count.get(&target) {
                    if cnt2 >= cnt {
                        let triplet = Self::normalize_triplet([nums[i], nums[j], target]);
                        ans_set.insert(triplet);
                    }
                }
            }
        }

        return ans_set.into_iter().map(|v| v.to_vec()).collect();
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
