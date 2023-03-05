
pub fn gcd<T>(mut a: T, mut b: T) -> T where T: std::cmp::PartialOrd + std::ops::Rem<Output=T> + Default + Copy {
    while b != T::default() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}