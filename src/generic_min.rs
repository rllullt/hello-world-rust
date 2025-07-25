use std::cmp::Ordering;

// TODO: implement the `min` function used in the tests.
fn min<T>(a: T, b: T) -> T
where
    T: Ord,
{
    // let order = a.cmp(&b);
    // match order {
    //     Ordering::Less => a,
    //     Ordering::Equal => a,
    //     Ordering::Greater => b,
    // }

    // Mejor (de la solución):
    match a.cmp(&b) {
        Ordering::Less | Ordering::Equal => a,
        Ordering::Greater => b,
    }
}

#[test]
fn integers() {
    assert_eq!(min(0, 10), 0);
    assert_eq!(min(500, 123), 123);
}

#[test]
fn chars() {
    assert_eq!(min('a', 'z'), 'a');
    assert_eq!(min('7', '1'), '1');
}

#[test]
fn strings() {
    assert_eq!(min("hello", "goodbye"), "goodbye");
    assert_eq!(min("bat", "armadillo"), "armadillo");
}