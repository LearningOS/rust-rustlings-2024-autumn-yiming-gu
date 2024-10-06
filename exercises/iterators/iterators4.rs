// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


struct Fac {
    num: i64,
    fac: i64,
}

impl Fac {
    fn new(n: i64) -> Fac {
        Fac{num: n, fac: 1}
    }
}

impl Iterator for Fac {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        if self.num == 0 {
            self.num -= 1;
            return Some(self.fac);
        }
        self.fac *= self.num;
        self.num -= 1;
        if self.num >= 0 {
            Some(self.fac)
        }
        else {
            None
        }
    }
}

pub fn factorial(num: i64) -> i64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.
    Fac::new(num).last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
