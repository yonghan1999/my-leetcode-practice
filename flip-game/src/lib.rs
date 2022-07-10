
struct Solution{}

impl Solution {
    pub fn generate_possible_next_moves(current_state: String) -> Vec<String> {
        let mut res = vec![];
        let s = current_state.as_bytes();
        let n = s.len() - 1;

        for i in 0..n {
            if s[i] == b'+' && s[i+1] == b'+' {
                res.push(current_state[0..i].to_string() + "--" + &current_state[i+2..])
            }
        }

        res
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
