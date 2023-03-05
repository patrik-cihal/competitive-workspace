use super::modulo::mod_inv;

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