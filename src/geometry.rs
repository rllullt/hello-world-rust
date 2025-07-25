// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.

fn magnitude(v: &[f64]) -> f64 {
    let mut sum = 0.0;
    for n in v {
        sum += n*n;
    }
    sum.sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.

fn normalize(v: &mut [f64]) {
    let mag = magnitude(v);
    for i in 0..v.len() {
        v[i] /= mag;
    }
    // // Can be also:
    // for item in v {
    //     *item /= mag;
    // }
}

// Use the following `main` to test your work.

pub fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}
