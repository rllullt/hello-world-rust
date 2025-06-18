pub fn fibo(n: u32) -> u32 {
    // Caso base
    if n == 0 || n == 1 {  // no son necesarios los paréntesis en este `if`
        return n;
    }
    fibo(n-1) + fibo(n-2)
}
