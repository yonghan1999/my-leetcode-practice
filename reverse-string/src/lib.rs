struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 {
            return;
        }

        let mut l = 0;
        let mut r = s.len() - 1;

        while l < r {
            let t = s[l];
            s[l] = s[r];
            s[r] = t;
            l += 1;
            r -= 1;
        }
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
