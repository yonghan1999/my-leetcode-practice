use std::collections::HashMap;

struct TwoSum {
    nums: HashMap<String,i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TwoSum {

    fn new() -> Self {
        Self {
            nums: HashMap::new()
        }
    }

    fn add(&mut self, number: i32) {
        let t = self.nums.get(number.to_string().as_str());
        match t {
            None => {
                self.nums.insert(number.to_string(),1);
            }
            Some(x) => {
                let a = x + 1;
                self.nums.insert(number.to_string(),a);
            }
        }
    }

    fn find(&self, value: i32) -> bool {
        let iter = self.nums.iter();
        for (k,v) in iter {
            let num = k.parse::<i32>().unwrap();
            let target = value - num;
            if target == num {
                if *v > 1 {
                    return true;
                }
                else {
                    continue;
                }
            }
            else if self.nums.get(target.to_string().as_str()).is_some() {
                return true;
            }
        }
        return false;
    }
}

/**
 * Your TwoSum object will be instantiated and called as such:
 * let obj = TwoSum::new();
 * obj.add(number);
 * let ret_2: bool = obj.find(value);
 */

#[cfg(test)]
mod tests {
    use crate::TwoSum;

    #[test]
    fn it_works() {
        let mut a = TwoSum::new();
        a.add(0);
        a.add(0);
        println!("{}",a.find(0));
    }
}
