impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
       let mut n = nums.len();
       let mut i=0;
       let mut j=0;
        while i < n{
            if nums[i] !=0{
                nums[j] = nums[i];
                j+=1;
            }
            i+=1;
        }
        
        while(j<n){
            nums[j] = 0;
            j+=1;
        }
    }
}
