#[cfg(test)]
struct Solution {}

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_unstable_by(|x, x1| {
            x[0].cmp(&x1[0])
        });
        let mut y = 0;
        for x in intervals.iter() {
            if y > x[0] {
                return false;
            }

            y = x[1];
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
