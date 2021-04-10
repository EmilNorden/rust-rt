use std::ops::{Mul, Add};

pub fn lerp<T>(a: T, b: T, factor: f32) -> T
    where
        T: Mul<f32, Output = T> + Add<T, Output = T>
{
    a * (1.0 - factor) + b * factor
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_should_return_first_value_when_factor_is_zero() {
        assert_eq!(500.0, lerp(500.0, 1000.0, 0.0));
    }

    #[test]
    fn lerp_should_return_second_value_when_factor_is_one() {
        assert_eq!(1000.0, lerp(500.0, 1000.0, 1.0));
    }

    #[test]
    fn lerp_should_return_middle_value_when_factor_is_half() {
        assert_eq!(750.0, lerp(500.0, 1000.0, 0.5));
    }
}