impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let mut first = 0;
        let mut second = numbers.len()-1;
        while first < second {
            if numbers[first]+numbers[second] == target{
                res.push(first as i32 + 1);
                res.push(second as i32 + 1);
                break;
            }
            if numbers[first]+numbers[second] < target{
                first +=1;
            }
            else{
                second -=1;
            }
        }
        res
        
    }
}
