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