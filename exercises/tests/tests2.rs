// tests2.rs
//
// This test has a problem with it -- make the test compile! Make the test pass!
// Make the test fail!
//
// Execute `rustlings hint tests2` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct Test {
    a : u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        let a = Test {
            a: 2
        };
        let b = Test {
            a: 2
        };
        assert_eq!(b, a);
    }
}
