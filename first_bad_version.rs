// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
    let mut l:i32 = 0;
    let mut h:i32 = n;
        while(l<=h){
            let mut mid = l + (h-l)/2;
            if self.isBadVersion(mid){
                h = mid - 1;
            }
            else{
                l = mid+1
            }
        }
        l
    }
}
