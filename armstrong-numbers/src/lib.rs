pub fn is_armstrong_number(num: u32) -> bool {

    let num_to_string = num.to_string();    
    let power = num_to_string.len() as u32;

    let v: Vec<Option<u32>> = num_to_string.chars()
    .map(|c| c.to_digit(10)).collect();

    let mut sum: u64=0;
    
    for i in v.iter(){
        let i = i.unwrap() as u64;
        sum += i.pow(power);
    }

    if sum == num.into(){
        return true;
    }

    false

}
