// Given a number `n`, return the `n+1`th number in the Fibonacci sequence.
//
// The Fibonacci sequence is defined as follows:
//
// - The first number of the sequence is 0.
// - The second number of the sequence is 1.
// - Every subsequent number is the sum of the two preceding numbers.
//
// So the sequence goes: 0, 1, 1, 2, 3, 5, 8, 13, 21, and so on.
//
// We expect `fibonacci(0)` to return `0`, `fibonacci(1)` to return `1`,
// `fibonacci(2)` to return `1`, and so on.

// pub fn fibonacci(n: u32) -> u32 {
//     static let mut fibo: Vec<u32> = vec![];
//     // TODO: implement the `fibonacci` function
//     //
//     // Hint: use a `Vec` to memoize the results you have already calculated
//     // so that you don't have to recalculate them several times.
//     if n == 0 {
//         if fibo.get(0) == None {
//             fibo.push(0);
//         }
//         return 0;
//     }
//     if n == 1 {
//         if fibo.get(1) == None {
//             fibo.push(1);
//         }
//         return 1;
//     }

//     if fibo.get(n as usize) == None {
//         fibo.push(fibonacci(n-1) + fibonacci(n-2));
//     }
//     return fibo[n as usize];
// }

// I am mad at this section. This problem is meaningless for the for vectors.
// If we want to do memo, the Vec should be outside of the function, or static.
// I'm writing my not Vec solution because it's just dumb.

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use crate::fibonacci;

    #[test]
    fn first() {
        assert_eq!(fibonacci(0), 0);
    }

    #[test]
    fn second() {
        assert_eq!(fibonacci(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(fibonacci(2), 1);
    }

    #[test]
    fn tenth() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn thirtieth() {
        assert_eq!(fibonacci(30), 832040);
    }
}
