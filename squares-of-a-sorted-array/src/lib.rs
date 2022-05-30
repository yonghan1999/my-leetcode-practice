use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut map: HashMap<i32, i32> = HashMap::new();
        for x in nums {
            if map.contains_key(&x.abs()) {
                res.push(*map.get(&x.abs()).unwrap());
            } else {
                let square = x.abs() * x.abs();
                map.insert(x.abs(), square);
                res.push(square);
            }
        }
        res.sort();
        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let t = vec![1,2,3,4,5,6,7];
        Solution::sorted_squares(t);
    }
}
