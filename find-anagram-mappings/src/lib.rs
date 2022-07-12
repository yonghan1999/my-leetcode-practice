use std::collections::HashMap;
impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        nums2.iter().enumerate().for_each(|(i, &n)| {
            map.insert(n, i as i32);
        });
        let mut ans = vec![];
        nums1.iter().for_each(|n| ans.push(map[n]));
        ans
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
