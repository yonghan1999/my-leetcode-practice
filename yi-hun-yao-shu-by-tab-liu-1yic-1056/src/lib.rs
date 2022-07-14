impl Solution {
    pub fn confusing_number(mut n: i32) -> bool {
        let mut num = n;
        let mut ans = 0;
        while num > 0 {
            match num % 10 {
                6 => ans = ans * 10 + 9,
                9 => ans = ans * 10 + 6,
                x @ (0 | 1 | 8) => ans = ans * 10 + x,
                _ => return false,
            }
            num /= 10;
        }
        ans != n
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
