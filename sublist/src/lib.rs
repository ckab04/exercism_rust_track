#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    //unimplemented!("Determine if the first list is equal to, sublist of, superlist of or unequal to the second list.");

    let first_list: HashSet<&T> = _first_list.iter().collect();
    let second_list: HashSet<&T> = _second_list.iter().collect();
    let len_f = first_list.len();
    let len_s = second_list.len();

    let value1 = second_list.iter().all(|v| first_list.contains(v));

    if (value1 ==true && len_f == len_s){
        return Equal;
    }



}
