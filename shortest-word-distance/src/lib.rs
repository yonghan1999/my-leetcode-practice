
struct Solution{}

impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let mut n = -1;
        let mut m = -1;
        let mut ans = words_dict.len() as i32;

        for (i, w) in words_dict.into_iter().enumerate() {
            if w == word1 {
                n = i as i32;
            } else if w == word2 {
                m = i as i32;
            }

            if n != -1 && m != -1 {
                ans = ans.min((n - m).abs());
            }
        }

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
