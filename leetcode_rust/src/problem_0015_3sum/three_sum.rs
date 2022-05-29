pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //
use std::collections::{HashMap, HashSet};

impl Solution {
    // three numbers sum to zero
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut flag = false;
        for num in &nums {
            if *num != 0 {
                flag = true;
            }
        }
        if flag {
            return vec![vec![0, 0, 0]];
        }

        let mut result_value_set: HashSet<Vec<i32>> = HashSet::new();
        let mut nums: Vec<i32> = nums.clone();
        let mut handled_value: HashSet<i32> = HashSet::new();
        for pos in 0..nums.len() {
            match handled_value.get(&nums[pos]) {
                None => {
                    println!("None");
                    let two_sum_res = Self::two_sum(&nums,  -nums[pos]);
                    for temp_res in two_sum_res {
                        let mut temp_set: HashSet<usize> = HashSet::new();
                        temp_set.insert(pos);
                        if temp_res.len() == 2 {
                            temp_set.insert(temp_res[0]);
                            temp_set.insert(temp_res[1]);

                            if temp_set.len() == 3 {
                                let mut value_vec: Vec<i32> = vec![];
                                for temp in temp_set {
                                    value_vec.insert(0, nums[temp]);
                                }
                                value_vec.sort();
                                result_value_set.insert(value_vec);
                                handled_value.insert(nums[pos]);
                            }

                        }
                    }
                }
                Some(_) => { }
            }
        }
        result_value_set.into_iter().collect()
    }
    // tow sum
    fn two_sum(sorted_nums: &Vec<i32>, target_num: i32) -> Vec<Vec<usize>>{
        let mut pos_vec_set: HashSet<Vec<usize>> = HashSet::new();
        let mut cache_map: HashMap<i32, Vec<usize>> = HashMap::new();

        for left in 0..sorted_nums.len() {
            println!("left: {}", left);
            match cache_map.get(&sorted_nums[left]) {
                None => {
                    cache_map.insert(sorted_nums[left], vec![left]);
                }
                Some(old_vec) => {
                    let mut new_vec: Vec<usize> = vec![left];
                    for num in old_vec {
                        new_vec.insert(0, *num);
                    }
                    cache_map.insert(sorted_nums[left], new_vec);
                }
            }
            let target = target_num - sorted_nums[left];
            match cache_map.get(&target) {
                None => {
                }
                Some(vec) => {
                    for right in vec {
                        pos_vec_set.insert(vec![left, *right]);
                    }
                }
            }
        }
        pos_vec_set.into_iter().collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Self::three_sum(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        print!("test_solution...");
        super::super::tests::run::<super::Solution>();
    }
}