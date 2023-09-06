mod hash_map;
mod my_stack;
mod my_rc;
#[allow(unused_imports)]
use my_stack::SimpleStack;
#[allow(unused_imports)]
use my_rc::MyRc;
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_stack() {
        let stack = SimpleStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_hash_map() {
        let map =
            hash_map! {
            "one" => 1,
            "two" => 2,
            "three" => 3,
        };
        assert_eq!(map["one"], 1);
        assert_eq!(map["two"], 2);
        assert_eq!(map["three"], 3);
    }

    #[test]
    fn test_my_rc() {
        let rc = MyRc::new(1);
        let rc2 = rc.clone();
        assert_eq!(rc.strong_count(), 2);
        assert_eq!(rc2.strong_count(), 2);
        assert_eq!(*rc, 1);
        assert_eq!(*rc2, 1);
        println!("rc: {:?}, rc2: {:?}", *rc, *rc2);
    }
}
