pub fn factorize(n: usize) -> Vec<(usize, usize)> {
    let mut n = n;
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        let mut cnt = 0;
        while n % i == 0 {
            cnt += 1;
            n /= i;
        }
        if cnt > 0 {
            res.push((i, cnt));
        }
        i += 1;
    }
    if n > 1 {
        res.push((n, 1));
    }
    res
}

pub fn sieve(n: usize) -> Vec<bool> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }
    is_prime
}
