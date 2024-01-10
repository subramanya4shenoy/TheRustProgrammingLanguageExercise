
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        let result = 2 + 1;
        assert_eq!(3, result);
    }

    #[test]
    fn another() {
        let checking_asert = true;
        // panic!("make it fail!");
        assert!(checking_asert);
    }

    #[test]
    fn test2() {
        let a = 3;
        assert_ne!(a, 4);
    }

    #[test]
    fn msg() {
        let a = 39;
        // assert_eq!(40, a, "Not euquals to 39");
    }

    fn test_panic() {
        panic!("wow i Panicked!")
    }
    
    #[test]
    #[should_panic]
    fn check_panic () {
        test_panic();
    }

    #[test]
    #[should_panic(expected = "wow i Panicked!")]
    fn check_panic_again () {
        test_panic();
    }

    #[test]
    fn run_only_this() {
        let a = 2;
        assert_eq!(a, 2); // run this with cargo test run_only_this
    }

    #[test]
    fn and_this() {
        let b = 4;
        assert_eq!(b, 4); // used for running multiple tests cargo test run _this 
    }

    #[test]
    #[ignore]
    fn ignore_this() {
        let c = 10;
        assert_eq!(c, 10); // this test gets ignored. we can run only ignored with cargo test -- --ignored
    }
} 