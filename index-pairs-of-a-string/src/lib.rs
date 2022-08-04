use std::cmp::Ordering;

struct Solution{}

impl Solution {
    pub fn index_pairs(text: String, words: Vec<String>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        for i in 0..text.len() {
            let substr = &text[i..text.len()];
            for x in  words.iter() {
                if substr.starts_with(x) {
                    let start = i as i32;
                    let end = start + x.len() as i32 - 1;
                    let item = vec![start,end];
                    res.push(item);
                }
            }
        }
        res.sort_by(|x1, x2| {
           if x1[0] - x2[0] > 0 {
               Ordering::Greater
           }
            else if x1[0] - x2[0] < 0 {
                Ordering::Less
            }
            else {
                if x1[1] - x2[1] > 0 {
                    Ordering::Greater
                }
                else if x1[1] - x2[1] < 0 {
                    Ordering::Less
                }
                else {
                    Ordering::Greater
                }
            }
        });
        return res;
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        let test = "thestoryofleetcodeandme".to_string();
        let words: Vec<String> = vec!["story".to_string(), "fleet".to_string(), "leetcode".to_string()];
        let res = Solution::index_pairs(test, words);
        for x in res.iter() {
            for x in x.iter() {
                println!("{}",x);
            }
        }
    }
}
