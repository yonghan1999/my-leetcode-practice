#[cfg(test)]
struct Solution {}


// 双指针
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                if i != j {
                    nums[i] = 0;
                }
                j+=1;
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
