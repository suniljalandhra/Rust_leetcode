impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 && nums[0] == target {
            return 0;
        }
        let mut l = 0;
        let mut h = nums.len() as i32 - 1;
        while(l <= h){
            let mid = (l+h)/2;
            if nums[mid as usize] == target{
                return mid;
            }
            if nums[mid as usize] < target{
                l = mid+1;
            }
            else {
                h = mid-1;
            }
        }
        -1
    }
}
