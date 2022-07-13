impl Solution {
    pub fn similar_rgb(color: String) -> String {
        let r = Self::get_num(&color, 1, 2);
        let g = Self::get_num(&color, 3, 4);
        let b = Self::get_num(&color, 5, 6);

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn get_num(color: &str, start: usize, end: usize) -> i32 {
        let c = i32::from_str_radix(color.get(start..=end).unwrap(), 16).unwrap();
        let vec = vec![0, 17, 34, 51, 68, 85, 102, 119, 136, 153, 170, 187, 204, 221, 238, 255];
        let mut ans = i32::MAX;

        for i in vec.into_iter() {
            ans = if (ans - c).abs() < (i - c).abs() {
                ans
            } else {
                i
            };
        }

        return ans
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
