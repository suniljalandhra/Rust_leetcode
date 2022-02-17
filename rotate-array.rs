impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, mut k: i32) {
        while k > 0{
            let mut prev:i32 = 0;
            let mut i:usize = 0;
            while i<nums.len()  {
                let mut curr:i32 = nums[i];
                nums[i] = prev;
                prev = curr;
                i+=1;
            }
            k-=1;
            nums[0] = prev;
        }
        
    }
}
