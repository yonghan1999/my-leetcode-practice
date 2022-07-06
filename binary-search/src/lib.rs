struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        loop {
            let index = (l + r) / 2;
            if *nums.get(index).unwrap() == target {
                return index as i32;
            } else if l == r {
                return -1;
            } else if *nums.get(index).unwrap() > target && index > l {
                r = index - 1;
            } else if *nums.get(index).unwrap() < target && index < r {
                l = index + 1;
            }
            else {
                return -1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        println!("{}",Solution::search(vec![2,5],0));
    }
}
