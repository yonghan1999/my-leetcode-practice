
struct MovingAverage {
    nums:Vec<i32>,
    size:i32,
    sum:i32
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovingAverage {

    fn new(size: i32) -> Self {
        Self {
            nums: Vec::new(),
            size,
            sum: 0
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        if self.size == 0 {
            return 0 as f64;
        }
        else {
            if self.nums.len() < self.size as usize {
                self.nums.push(val);
                self.sum += val;
            }
            else {
                let first = self.nums.first().unwrap();
                self.sum -= first;
                self.nums.remove(0);
                self.nums.push(val);
                self.sum += val;
            }
            return self.sum as f64 / self.nums.len() as f64;
        }
    }
}

/**
 * Your MovingAverage object will be instantiated and called as such:
 * let obj = MovingAverage::new(size);
 * let ret_1: f64 = obj.next(val);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
