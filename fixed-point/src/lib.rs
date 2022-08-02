struct Solution {}
impl Solution {
    pub fn fixed_point(arr: Vec<i32>) -> i32 {
        for (i,x) in arr.iter().enumerate() {
            if *x == i as i32 {
                return i as i32;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
