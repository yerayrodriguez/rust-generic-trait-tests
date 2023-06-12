use calculator_trait::{calculator::Calculator, test_calculator_trait};

pub struct CalculatorImplB {}

impl Calculator for CalculatorImplB {
    fn new() -> Self {
        Self {}
    }

    fn add(&self, a: u32, b: u32) -> u32 {
        if b == 0 {
            a
        } else {
            self.add(a, b - 1) + 1
        }
    }
}

test_calculator_trait! {
    unit_test_impl_b:CalculatorImplB
}
