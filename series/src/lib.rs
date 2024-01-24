pub fn series(digits: &str, len: usize) -> Vec<String> {
    //("What are the series of length {len} in string {digits:?}")

    //let dig: Vec<char> = digits.chars().collect();
    //let string = String::new();
    let digits_len = digits.len();

    if digits.is_empty() || len > digits_len{
        return Vec::new();
    }

    let mut v_series = Vec::new();

    let length = (digits.len() - len) + 1;

    //"49142" len: 3
    let mut l = len;

    for i in 0..length{        
        let s = &digits[i..l];
        v_series.push(s.to_string());
        if l < digits.len(){
            l +=1;
        }
        
    }

v_series

}
