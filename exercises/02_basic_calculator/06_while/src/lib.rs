// Rewrite the factorial function using a `while` loop.
pub fn factorial_1(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.
    let mut res = 1;
    let mut counter = 1;
    while counter <= n {
        res *= counter;
        counter += 1;
    }
    return res
}

pub fn factorial(mut n: u32) -> u32 {
    let mut res = 1;
    while n > 0 {
        res *= n;
        n -= 1;
    }
    return res
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
