use std::cmp::max;

fn knapsack(wt:Vec<i32>,val:Vec<i32>,size: usize,cap: i32) -> i32{
    if size == 0||cap == 0 {
        return 0;
    }
    if wt[size-1]<=cap {
        return max(val[size -1] + knapsack(wt,val,size-1,cap-wt[size-1]), knapsack(wt, val, size-1, cap));
    }
    else{
        return knapsack(wt, val, size-1, cap);
    }
    
}
fn main(){
    println!("hello world");
}