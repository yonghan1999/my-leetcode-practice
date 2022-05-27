struct Solution {}

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;

        let version = (l + r) / 2;
        let mut firstCheck = self.isBadVersion(version);
        if firstCheck {
        }
        loop {
            let version = (l + r) / 2;
            if self.isBadVersion(version) && version > l{
                r = version - 1;
            }
            else {
                return version;
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
