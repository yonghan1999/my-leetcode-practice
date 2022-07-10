struct Solution{}

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        let word = word.chars().collect::<Vec<_>>();

        let mut step = 0;
        let mut idx = -1i32 as usize;
        for c in abbr.bytes() {
            if c == b'0' && step == 0 {
                return false;
            }
            if c >= b'0' && c <= b'9' {
                step = step * 10 + (c - b'0') as usize;
            } else {
                idx += step + 1;
                if idx >= word.len() {
                    return false;
                }
                if word[idx] != c as char {
                    return false;
                }
                step = 0;
            }
        }
        word.len() == idx + step + 1
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
