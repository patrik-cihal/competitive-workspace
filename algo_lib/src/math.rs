pub mod primes;

pub fn gcd<T>(mut a: T, mut b: T) -> T where T: std::cmp::PartialOrd + std::ops::Rem<Output=T> + Default + Copy {
    while b != T::default() {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T where T: std::cmp::PartialOrd + std::ops::Rem<Output=T> + Default + Copy + std::ops::Mul<Output=T> + std::ops::Div<Output=T> {
    a / gcd(a, b) * b
}


pub fn mod_pow(base: u64, exponent: u64, modulo: u64) -> u64 {
    let mut result = 1;
    let mut base = base;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = result * base % modulo;
        }
        base = base * base % modulo;
        exponent /= 2;
    }
    result
}

pub fn mod_inv(x: u64, modulo: u64) -> u64 {
    mod_pow(x, modulo - 2, modulo)
}

pub fn nck_mod_p(n: u64, k: u64, p: u64) -> u64 {
    if k > n {
        return 0;
    }
    if k * 2 > n {
        return nck_mod_p(n, n - k, p);
    }
    let mut numerator = 1;
    let mut denominator = 1;
    for i in 0..k {
        numerator = numerator * (n - i) % p;
        denominator = denominator * (i + 1) % p;
    }
    numerator * mod_inv(denominator, p) % p
}