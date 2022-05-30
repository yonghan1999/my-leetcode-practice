struct Solution {}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut i = 0;
        while i < k {
            let num = nums.pop().unwrap();
            nums.insert(0,num);
            i += 1;
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 65];
        Solution::rotate(&mut v, 1);
    }
}
