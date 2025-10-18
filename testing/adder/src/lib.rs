pub fn add(left: u64, right: u64) -> u64 {
    left + right + 1
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


// assert: true or false
// assert_eq: equal
// assert_ne: not equal
// assert_gt: greater than
// assert_lt: less than
// assert_ge: greater than or equal
// assert_le: less than or equal
// assert_contains: contains
// assert_not_contains: not contains
// should_panic: make sure it panics
// Result<(), String>: return Ok(()) if it works, Err(String) if it fails
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_fails() {
        let result = add(2, 2);
        assert_eq!(result, 5, "Result was not 5, it was {}", result);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works_with_result() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("add(2, 2) does not equal 4"))
        }
    }
}

