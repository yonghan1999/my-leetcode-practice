struct Solution{}

impl Solution {
    pub fn are_sentences_similar(sentence1: Vec<String>, sentence2: Vec<String>, similar_pairs: Vec<Vec<String>>) -> bool {
        if sentence1.len() != sentence2.len() {
            return false;
        }

        let s = similar_pairs.iter().map(|x| (&x[0], &x[1])).collect::<std::collections::HashSet<_>>();

        for i in 0..sentence1.len() {
            if sentence1[i] != sentence2[i] &&  !s.contains(&(&sentence1[i], &sentence2[i])) && !s.contains(&(&sentence2[i], &sentence1[i])) {
                return false;
            }
        }

        true
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
