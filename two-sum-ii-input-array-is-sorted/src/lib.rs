struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            let sum = numbers[l] + numbers[r];
            if target == sum {
                break;
            } else if target > sum {
                l += 1;
            } else {
                r -= 1;
            }
        }
        vec![l as i32 + 1, r as i32 + 1]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        Solution::two_sum(vec![2,7,11,15],9);
    }
}
