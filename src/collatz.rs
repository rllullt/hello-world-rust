/// Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:
/// If ni is 1, then the sequence terminates at ni.
/// If ni is even, then ni+1 = ni / 2.
/// If ni is odd, then ni+1 = 3 * ni + 1.

/// Determine the length of the collatz sequence beginning at `n`.
pub fn collatz_length(mut n: i32) -> u32 {
    let mut i = 1;
    while n != 1 {
        n = if n % 2 == 0 { n / 2  } else { 3 * n + 1 };
        i += 1;
    }
    i
}
