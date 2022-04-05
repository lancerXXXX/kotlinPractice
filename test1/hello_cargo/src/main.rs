use core::num;
use std::{collections::{HashMap}, vec};

fn main() {
    println!("Hello, world!");
    let sl: Solution = Solution {};
    let nums: Vec<i32> = vec![-1,0,1,2,-1,-4];
    sl.three_sum(nums);
}

struct Solution;

impl Solution {
    // three numbers sum to zero
    pub fn three_sum(&self, nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        let mut nums = nums.clone();
        nums.sort();
        for (pos, num) in nums.iter().enumerate() {
            let target_right = -num;
            let two_sum_res = self.two_sum(&nums, target_right, pos as i32 + 1);
            for res in two_sum_res {
                result.push(vec![nums[pos], res[0], res[1]]);
            }
        }
        result
    }
    // two numbers sum to target
    pub fn two_sum(&self, sorted_nums: & Vec<i32>, target: i32, begin: i32) -> Vec<Vec<i32>> {
        // 最少也要两个数
        let end: i32 = sorted_nums.len() as i32 - 1;
        if begin >= end || end - begin < 1 {
            return vec![];
        }
        let mut num_pos_map: HashMap<i32, i32> = HashMap::new();
        let mut result:  Vec<Vec<i32>> = vec![];
        // 上一个需要处理的left数字位置
        let mut pre_left_num_pos: i32 = -1;

        for pos in begin..end {
            // 如果和上一个处理的left数字相同，则不需要处理
            if pre_left_num_pos != -1 && sorted_nums[pos as usize] == sorted_nums[pre_left_num_pos as usize] {
                continue;
            }
            let ept_right_num = target - sorted_nums[pos as usize];
            if let Some(j) = num_pos_map.get(&ept_right_num) {
                result.push(vec![sorted_nums[pos as usize], ept_right_num]);
                pre_left_num_pos = pos;
            }
            num_pos_map.insert(sorted_nums[pos as usize], pos);
        }
        result
    }
}
