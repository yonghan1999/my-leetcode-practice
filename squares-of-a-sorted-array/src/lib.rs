use std::collections::HashMap;

struct Solution {}

// impl Solution {
//     pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//         let mut res: Vec<i32> = Vec::new();
//         let mut map: HashMap<i32, i32> = HashMap::new();
//         for x in nums {
//             if map.contains_key(&x.abs()) {
//                 res.push(*map.get(&x.abs()).unwrap());
//             } else {
//                 let square = x.abs() * x.abs();
//                 map.insert(x.abs(), square);
//                 res.push(square);
//             }
//         }
//         res.sort();
//         return res;
//     }
// }


// 双指针，比上面的那个垃圾暴力，聪明有能干
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut i,mut j,mut k) = (0,n - 1,n- 1);
        let mut ans = vec![0;n];
        while i <= j{
            if nums[i] * nums[i] < nums[j] * nums[j] {
                ans[k] = nums[j] * nums[j];
                j -= 1;
            }else{
                ans[k] = nums[i] * nums[i];
                i += 1;
            }
            k -= 1;
        }
        ans
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
