use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    //todo!("Sum the multiples of all of {factors:?} which are less than {limit}")

    let mut mult: HashSet<u32> = HashSet::new();

    for i in factors{
        let v = multiple_of_number(limit, *i);
        //println!("Values in the vector : {:?}", v);

        mult.extend(v.iter());
    }

    println!("Value in the hashSet : {:?}", mult);

    mult.iter().sum()

}
//let factors = &[3, 5];
//let limit = 1;

fn multiple_of_number(limit: u32, number: u32) -> Vec<u32> {

    if number == 0{
        return Vec::new()
    }

    let mut array: Vec<u32> = Vec::new();
    

    //array.push(number);
    for i in 1..limit{
        if i % number == 0{
            array.push(i);
            //println!("Pushed value = {}", i);
        }
    }

    array
    
}