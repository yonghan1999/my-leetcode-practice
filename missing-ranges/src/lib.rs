use std::iter;

struct Solution {}

impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let mut  start = lower;
        let mut res:Vec<String> = Vec::new();

        let iter = nums.iter();
        for x in iter {
            let mut s:String;
            if *x > start && *x <= upper {
                if *x > start + 1 {
                    s = (start.to_string() + "->" + &*(*x - 1).to_string());
                }
                else {
                    s = start.to_string();
                }
                start = *x + 1;
                res.push(s);
            }
            else {
                start = start + 1;
            }
        }
        if start <= upper {
            if upper >= start + 1 {
                res.push((start.to_string() + "->" + &*upper.to_string()));
            }
            else {
                res.push(start.to_string());
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        Solution::find_missing_ranges(vec![-1],-2,-1);
    }
}
