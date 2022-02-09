fn main() {
    let x = is_palindrome(121);
   println!(" output is {}",x);
}
fn is_palindrome( x: i32) -> bool {
    let mut p = x.clone();
    if p < 0 {
        return false;
    }
    let mut a:i32 = 0;
    loop {
        if p == 0{
            break;
        }
        a = a*10 + p%10;
        p = p/10;
    }
    println!("{}",a);
    return a == x;
}