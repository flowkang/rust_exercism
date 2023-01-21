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
    let mut buffer = create_buffer(5);
    buffer[0] = 1;
    buffer[1] = 1;
    for i in 2..5 {
        buffer[i] = buffer[i-1] + buffer[i-2];
    }
    buffer
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let empty = create_empty();
        assert_eq!(empty, []);
        let my_buffer = create_buffer(5);
        assert_eq!(my_buffer, [0, 0, 0, 0, 0]);
        let first_five = fibonacci();
        assert_eq!(first_five, [1, 1, 2, 3, 5]);
    }
}
