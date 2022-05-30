#[cfg(test)]
struct Solution {}


// 魔改冒泡排序
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] == 0 {
                    let t = nums[i];
                    nums[i] = nums[j];
                    nums[j] = t;
                }
                else { break; }
            }
        }
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut v = vec![1, 0, 0, 2, 4, 46, 87, 4];
        Solution::move_zeroes(&mut v);
        println!("{:?}", v);
    }
}
