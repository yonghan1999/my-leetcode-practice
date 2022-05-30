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
// impl Solution {
//     pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//         let n = nums.len();
//         let (mut i,mut j,mut k) = (0,n - 1,n- 1);
//         let mut ans = vec![0;n];
//         while i <= j{
//             if nums[i] * nums[i] < nums[j] * nums[j] {
//                 ans[k] = nums[j] * nums[j];
//                 j -= 1;
//             }else{
//                 ans[k] = nums[i] * nums[i];
//                 i += 1;
//             }
//             if k > 0 {
//                 k -= 1;
//             }
//         }
//         ans
//     }
// }


// 空间换时间 简单优化一下？
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let (mut i,mut j,mut k) = (0,n - 1,n- 1);
        let mut ans = vec![0;n];
        let mut map: HashMap<i32, i32> = HashMap::new();
        while i <= j{
            if nums[i].abs() < nums[j].abs() {
                if map.contains_key(&nums[j].abs()) {
                    ans[k] = *map.get(&nums[j].abs()).unwrap()
                } else {
                    let square = nums[j] * nums[j];
                    map.insert(nums[j].abs(), square);
                    ans[k] = square;
                }
                j -= 1;
            }else{
                if map.contains_key(&nums[i].abs()) {
                    ans[k] = *map.get(&nums[i].abs()).unwrap()
                } else {
                    let square = nums[i] * nums[i];
                    map.insert(nums[i].abs(), square);
                    ans[k] = square;
                }
                i += 1;
            }
            if k > 0 {
                k -= 1;
            }
        }
        ans
    }
}


#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let t = vec![-4,-1,0,3,10];
        Solution::sorted_squares(t);
    }
}
