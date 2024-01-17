#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    unimplemented!(
		"Given the inputs: {:?}, evaluate them as though they were a Reverse Polish notation expression",
		inputs,
	);
    let mut v = vec![];
   for i in inputs.iter(){
    if i.is_ascii_digit()
    {
        v.push(i);
        Some(i)
    }
    else
    {
       None
    }
   }

None

}
