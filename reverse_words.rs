impl Solution {
    pub fn reverse_words(mut s: String) -> String {
        
        s.split_whitespace()
        .map(|word|word.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
    }
}
