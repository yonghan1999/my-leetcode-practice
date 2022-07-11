struct StringIterator {
    index: usize,
    count: Vec<usize>,
    data: Vec<char>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StringIterator {

    fn new(compressedString: String) -> Self {
        let mut nums = 0;
        let mut digit = 1;
        let mut count = vec![];
        let mut data = vec![];

        for ch in compressedString.chars().rev() {
            if ch >= '0' && ch <= '9' {
                nums = (ch as usize - 48) * digit + nums;
                digit *= 10;
            } else {
                data.push(ch);
                count.push(nums);
                nums = 0;
                digit = 1;
            }
        }

        Self {
            index: count.len() - 1,
            count,
            data,
        }
    }

    fn next(&mut self) -> char {
        let count = self.count[self.index];

        if count > 0 {
            self.count[self.index] -= 1;
            return self.data[self.index];
        }

        if self.index == 0 {
            return ' ';
        }

        self.index -= 1;
        let count = self.count[self.index];
        self.count[self.index] -= 1;
        return self.data[self.index];
    }

    fn has_next(&self) -> bool {
        self.index > 0 || self.count[0] > 0
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
