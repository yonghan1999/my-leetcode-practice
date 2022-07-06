
/**
 * The read4 API is defined as.
 *     fn read4(&self,buf4: &mut [char]) -> i32;
 * You can call it using self.read4(buf4)
 */

struct Solution {}

impl Solution {
    pub fn read(&self, buf: &mut [char], n: i32) -> i32 {
        let mut buf_index = 0;
        loop {
            let mut cache:[char;4] = [' '; 4];
            let t = self.read4(&mut cache);
            let is_last = if t < 4 {
                true
            } else {
                false
            };
            let mut i = 0;
            while i < t {
                buf[buf_index] = cache[i as usize];
                i += 1;
                buf_index += 1;
                if n == (buf_index as i32) {
                    return buf_index as i32;
                }
            }
            if is_last {
                return buf_index as i32;
            }
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
/*
Line 12, Char 32: borrow of possibly-uninitialized variable: `cache` (solution.rs)
   |
12 |             let t = self.read4(&mut cache);
   |                                ^^^^^^^^^^ use of possibly-uninitialized `cache`
For more information about this error, try `rustc --explain E0381`.
error: could not compile `prog` due to previous error
mv: cannot stat '/leetcode/rust_compile/target/release/prog': No such file or directory
*/