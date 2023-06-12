use crate::{calculator::Calculator, test_calculator_trait};

pub struct CalculatorImplA {}

impl Calculator for CalculatorImplA {
    fn new() -> Self {
        Self {}
    }

    fn add(&self, a: u32, b: u32) -> u32 {
        a + b
    }
}

test_calculator_trait! {
    unit_test_impl_a:CalculatorImplA
}
