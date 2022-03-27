use core::num;
use std::{hash::{Hash, Hasher}, collections::{HashSet, hash_map::DefaultHasher}};

fn main() {
    println!("Hello, world!");
    let sl: Solution = Solution {};
    let nums: Vec<i32> = vec![1, 2, 3, -1, -2];

    let mut hs1 = DefaultHasher::new();
    let mut hs2 = DefaultHasher::new();
    let test1: Vec<i32> = vec![1, 2, 3];
    let test2: Vec<i32> = vec![1, 2, 3];
    test1.hash(&mut hs1);
    test2.hash(&mut hs2);
    hs1.finish();
    println!("{}", hs1.finish());
    println!("{}", hs2.finish());

    sl.three_sum(nums);
}

struct Solution;

impl Solution {
    pub fn three_sum(&self, nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut v_1 = 0;
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut res2: HashSet<Vec<i32>> = HashSet::new();
        loop {
            if v_1 > nums.len() - 3 {
                break;
            }
            let mut v_2 = v_1 + 1;
            loop {
                if v_2 > nums.len() - 2 {
                    break;
                }
                let mut v_3 = v_2 + 1;
                loop {
                    if v_3 > nums.len() - 1 {
                        break;
                    }
                    if nums[v_1] + nums[v_2] + nums[v_3] == 0 {
                        res.insert(0, Vec::from([nums[v_1], nums[v_2], nums[v_3]]))
                    }
                    v_3 += 1;
                }
                v_2 += 1;
            }
            v_1 += 1;
        }
        return res;
    }
}
