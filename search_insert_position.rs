use std::cmp::Ordering;
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut h = nums.len() - 1;
       if nums[l] > target {
           return 0;
        }
        while(l<=h){
            let mid = l + (h-l)/2;
            match nums[mid].cmp(&target) {
                Ordering::Equal => {return mid as i32;},
                Ordering::Greater => {h = mid -1;},
                Ordering::Less => {l = mid+1;},
            }
        }
        l as i32
        
    }
}
