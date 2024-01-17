/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
    }

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    
    let mut value : Vec<u8> = vec![0; 5];
    value[0] = 1;
    value[1] = 1;
    let mut y = 2;
    for _ in 0..3
    {
        value[y] = value[y - 1] + value[ y - 2];
        y = y + 1;
              
    }
    println!("{:?}", value);
    value
}
