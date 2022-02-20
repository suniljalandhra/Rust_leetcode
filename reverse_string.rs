impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut first = 0;
        let mut second = s.len()-1;
        while first < second {
            let mut temp = s[first];
            s[first] = s[second];
            s[second] = temp;
            first +=1;
            second -=1;
        }
    }
}
