use super::divison::gcd;

pub fn lcm<T>(a: T, b: T) -> T where T: std::cmp::PartialOrd + std::ops::Rem<Output=T> + Default + Copy + std::ops::Mul<Output=T> + std::ops::Div<Output=T> {
    a / gcd(a, b) * b
}

