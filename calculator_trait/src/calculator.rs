pub trait Calculator {
    fn new() -> Self;
    fn add(&self, a: u32, b: u32) -> u32;
}

#[macro_export]
macro_rules! test_calculator_trait {
    ($name:ident: $type:ty) => {
        mod $name {
            #[allow(unused_imports)]
            use super::*;

            #[test]
            fn test_1() {
                let c = <$type>::new();
                assert_eq!(c.add(2, 3), 5);
            }

            #[test]
            fn test_2() {
                let c = <$type>::new();
                assert_eq!(c.add(6, 9), 15);
            }
        }
    };
}
