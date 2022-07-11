struct StringIterator {
    arr:Vec<char>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StringIterator {

    // 不能直接解析 超出时间显示，简单题这样好吗，这样不好 fk
    fn new(compressedString: String) -> Self {
        let mut parse_arr : Vec<char> = Vec::new();
        let mut nums = 0;
        let mut digit = 1;
        for ch in compressedString.chars().rev() {
            if ch >= '0' && ch <= '9' {
                nums = (ch as usize - 48) * digit + nums;
                digit *= 10;
            } else {
                for i in 0..nums{
                    parse_arr.push(ch);
                }
                nums = 0;
                digit = 1;
            }
        }
        Self {
            arr:parse_arr
        }
    }

    fn next(&mut self) -> char {
        return match self.arr.pop() {
            None => {
                ' '
            }
            Some(x) => {
                x
            }
        };
    }

    fn has_next(&self) -> bool {
        !self.arr.is_empty()
    }
}

/**
 * Your StringIterator object will be instantiated and called as such:
 * let obj = StringIterator::new(compressedString);
 * let ret_1: char = obj.next();
 * let ret_2: bool = obj.has_next();
 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
