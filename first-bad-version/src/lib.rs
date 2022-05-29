struct Solution {}

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut good: i64 = 0;
        let mut bad: i64 = n as i64;
        while bad - good > 1 {
            let mid = (good + bad + 1) / 2;
            if self.isBadVersion(mid as i32) {
                bad = mid;
            } else {
                good = mid;
            }
        }
        bad as i32
    }
}