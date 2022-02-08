use std::cmp::max;
static NUM: &'static [[i32;100];100] = &[[-1i32;100];100];
fn knapsack(wt:&Vec<i32>,val:&Vec<i32>,size: usize,cap: i32) -> i32{

    if size == 0||cap == 0 {
        return 0;
    }
    if()
    if wt[size-1]<=cap {
        return max(val[size -1] + knapsack(wt,val,size-1,cap-wt[size-1]), knapsack(wt, val, size-1, cap));
    }
    else{
        return knapsack(wt, val, size-1, cap);
    }
    
}
fn main(){
    let  wt = vec![1,3,4,5];
    let  val = vec![1,4,5,7];
    let  cap = 7;
    let  size:usize = 4;
    let  x = knapsack(&wt, &val, size, cap);
    println!("{}",x);
}