fn main(){
    let s = String::from("LVIII");
 let x = roman_to_int(s);
 println!("integer is {}",x);
}
fn roman_to_int(s: String) -> i32{
    s.chars().rfold(0,|acc, c| {
        acc + match c {
            'I' if acc >= 5 => -1,
            'I' => 1,
            'V' => 5,
            'X' if acc >= 50 => -10,
            'X' => 10,
            'L' => 50,
            'C' if acc >= 500 => -100,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,

        }
    })
}