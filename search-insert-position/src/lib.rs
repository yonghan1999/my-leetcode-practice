
struct Solution {}

use std::cmp::Ordering;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut size = nums.len();
        let mut left = 0;
        let mut right = size;
        while left < right {
            let mid = left + size / 2;
            let cmp = unsafe { nums.get_unchecked(mid) }.cmp(&target);
            if cmp == Ordering::Less {
                left = mid + 1;
            } else if cmp == Ordering::Greater {
                right = mid;
            } else {
                return mid as i32;
            }
            size = right - left;
        }
        return left as i32;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        print!("{}", Solution::search_insert(vec![1, 3], 0));
    }
}
