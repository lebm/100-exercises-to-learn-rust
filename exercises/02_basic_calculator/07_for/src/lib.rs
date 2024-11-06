// Rewrite the factorial function using a `for` loop.
#[allow(dead_code)]
pub fn factorial(n: u32) -> u32 {
    let mut f = 1;
    for i in 1..=n {
        f = f * i;
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
