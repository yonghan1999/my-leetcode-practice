use std::collections::HashMap;

#[cfg(test)]

struct Solution{}

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let mut map:HashMap<char,i32> = HashMap::new();
        s.chars().for_each(|x| {
            if map.contains_key(&x) {
                map.insert(x,map.get(&x).unwrap() + 1);
            }
            else {
                map.insert(x,1);
            }
        });
        let mut j = true;
        if s.len()%2 == 0 {
            j = false;
        }
        let mut count = 0;
        for x in map {
            if x.1 %2 != 0 {
                count += 1;
            }
        }
        if j {
            count -= 1;
        }
        return count <= 0;
    }
}


mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
