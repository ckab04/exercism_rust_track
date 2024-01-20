
#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::clone::Clone>(first_list: &[T], second_list: &[T]) -> Comparison {
    //("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");

    let len_first = first_list.len();
    let len_second = second_list.len();

    if len_first == len_second{
        let value = first_list.iter().zip(second_list.iter()).filter(|&(a,b)| a == b).count();
        if value == first_list.len() && value == second_list.len(){
            return Comparison::Equal;
        }else {
            return Comparison::Unequal;
        }
    }else if len_first > len_second {
        return first_list_comparison(first_list, second_list);
    }else if len_second > len_first {
        return second_list_comparison(first_list, second_list);
    }
    else {
        return Comparison::Unequal;
    }    

}


fn first_list_comparison<T: PartialEq  + std::clone::Clone>(first_list: &[T], second_list: &[T]) -> Comparison{

    let f = first_list.to_vec();
    let s = second_list.to_vec();
    let f_len = f.len();
    let s_len = s.len();

    if !first_list.is_empty() && second_list.is_empty(){
        return Comparison::Superlist;
    }
    let mut i = 0;
    let mut contiguous = false;

    //let mut first_list_elements = HashSet::from_iter(first_list.iter());
    //first_list_elements.extend(first_list.iter());
    let mut count = 0;
    for el in first_list.iter(){
       // if first_list.contains(el){
         if el == s.get(i).expect("Could not get the index"){
            count += 1;
            i += 1;
         }else{
            count = 0;
            i = 0;
         }
         if count == s_len{
            contiguous = true;
            break;
         }


            
      //  }
    }

    if contiguous && first_list.len() > count {       
        return Comparison::Superlist;
    }

    Comparison::Unequal

}


fn second_list_comparison<T: PartialEq + std::clone::Clone>(first_list: &[T], second_list: &[T]) -> Comparison{
    if first_list.is_empty() && !second_list.is_empty(){
        return Comparison::Sublist;
    }

    //let mut first_list_elements = HashSet::from_iter(first_list.iter());
    //first_list_elements.extend(first_list.iter());
    //let list_one: &[i32] = &[1, 2, 3];
    //let list_two: &[i32] = &[1, 3];
    //let list_one: &[i32] = &[1, 1, 2];
    //let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];

    let f = first_list.to_vec();
    let s = second_list.to_vec();

    let s_len = s.len();
    let f_len = f.len();
    let mut contiguous = false;

    let mut count = 0;
    let mut i =0;
    for  el in second_list.iter(){
       // if second_list.contains(el) 
            if el == f.get(i).expect("Cannot get the index"){
                println!("Value of i = {i}");
                count += 1;
                i += 1;
            }else{

                count = 0;
                i = 0;
            }         
            //}
        if f_len == count{
            contiguous = true;
            break;
        }
    }

    if contiguous && second_list.len() > count {
       
        return Comparison::Sublist;
        
    }

    Comparison::Unequal

}

