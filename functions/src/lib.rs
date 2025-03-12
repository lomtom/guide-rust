#[cfg(test)]
mod tests {

    fn add(left: i32, right: i32) -> i32 {
        left + right
    }

    #[test]
    fn func() {
        assert_eq!(add(2, 2), 4);
    }

    fn increase(left: i32, right: i32) -> (i32,i32) {
        (left + 1,right+1)
    }

    #[test]
    fn test_increase() {
        let (left,right) = increase(2,2);
        assert_eq!(left,3);
        assert_eq!(right,3);
    }

    #[test]
    fn test_closure() {
        let add = |left: i32, right: i32| left + right;
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_closure_env() {
        let x = 2;
        let add =  |left: i32, right: i32| left + right + x;
        assert_eq!(add(2, 2), 6);
    }
}
