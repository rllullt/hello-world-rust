pub fn an_array() {
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];  // an array of type ‘i8 with 5 elements’
    a[2] = 0;
    // println! macro receives:
    // {} for the default output format
    // {:?} for the debug output, it requires the type to implement the Debug trait
    // {:b} for binary format
    // {:x} for hexadecimal format
    // {:o} for octal format
    // {:e} for scientific notation
    // {:p} for pointer address
    // Arrays only implement debug output format
    println!("a: {a:?}");
    dbg!(a[4]);

    println!("Primes in array:");
    let primes = [2, 3, 5, 7, 11, 13, 17, 19];
    for prime in primes {
        dbg!(prime);
        for i in 2..prime {
            assert_ne!(prime % i, 0);
            // assert_ne! macro checks that the two values are not equal
            // assert_eq! macro checks that the two values are equal
            // debug_assert! macro checks that the expression is true, but only in debug mode (compiles to nothing in release builds)
        }
    }
}

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            new_matrix[j][i] = matrix[i][j];
        }
    }
    new_matrix
}
