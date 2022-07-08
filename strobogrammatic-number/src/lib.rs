use std::collections::HashMap;

#[cfg(test)]
struct Solution {}

impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        if num.is_empty() {
            return true;
        }
        let mut a: HashMap<char, char> = HashMap::new();
        a.insert('0', '0');
        a.insert('1', '1');
        a.insert('6', '9');
        a.insert('8', '8');
        a.insert('9', '6');
        for (i, c) in num.chars().enumerate() {
            if i > num.len() / 2 + 1 {
                return true;
            }
            if a.contains_key(&c) {
                let t = a.get(&c).unwrap();
                if num.chars().nth(num.len() - 1 - i).unwrap() != *t {
                    return false;
                }
                else {
                    continue;
                }
            }
            else {
                return false;
            }
        }
        return true;
    }
}

mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
