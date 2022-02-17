impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
     let mut r = k as usize % nums.len();
        nums.reverse();
        nums[0..r].reverse();
        nums[r..].reverse();
        
    }
}
