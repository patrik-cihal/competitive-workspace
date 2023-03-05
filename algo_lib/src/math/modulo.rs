
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