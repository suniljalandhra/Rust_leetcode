


fn main(){
    let a = add_option(4, Some(54));
    println!("{}",a);
}
fn add_option(i:i32,j:Option<i32>) -> i32{
    i + match j {
        Some(p) => p, 
        None => 0,
    }
}