// Rewrite the factorial function using a `while` loop.
fn factorial(n: u32) -> u32 {
    let mut temp = n;
    let mut f = 1;
    while temp > 0 {
        f *= temp;
        temp -= 1;
    }
    f
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
