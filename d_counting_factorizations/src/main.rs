//{"name":"D. Counting Factorizations","group":"Codeforces - Codeforces Round 856 (Div. 2)","url":"https://codeforces.com/contest/1794/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"2\n1 3 2 3\n","output":"2\n"},{"input":"2\n2 2 3 5\n","output":"5\n"},{"input":"1\n1 4\n","output":"0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCountingFactorizations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::primes::sieve;
use algo_lib::{out, out_line};
use algo_lib::math::modulo::mod_inv;

const MOD: u64 = 998_244_353;

fn inv(x: u64) -> u64 {
    mod_inv(x, MOD)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a = input.read_vec::<u64>(n*2);

    let mut fact = vec![1u64; n*2+1];
    for i in 1..2*n+1 {
        fact[i] = (i as u64 * fact[i-1]) % MOD;
    }

    a.sort();

    let is_prime = sieve(1_000_0001);


    let mut prime_counts = vec![];
    let mut non_prime_counts = vec![];

    for i in 0..n*2 {
        let mut v = &mut non_prime_counts;
        if is_prime[a[i] as usize] {
            v = &mut prime_counts;
        }
        if i>0 && a[i-1]==a[i] {
            let Some(last) = v.last_mut() else {
                panic!();
            };
            *last += 1;
        }
        else {
            v.push(1);
        }
    }

    let mut B = 1;
    for count in non_prime_counts {
        B = B * fact[count] % MOD; 
    }

    if prime_counts.len() < n {
        out_line!(0);
        return;
    }

    let constant = fact[n]*inv(B)%MOD;

    let mut dp: Vec<Vec<u64>> = vec![vec![0; n+1]; prime_counts.len()];

    dp[0][0] = inv(fact[prime_counts[0]]);
    dp[0][1] = inv(fact[prime_counts[0]-1]);
    for i in 1..prime_counts.len() {
        dp[i][0] = (dp[i-1][0] * inv(fact[prime_counts[i]]))%MOD;
        for j in 1..n+1 {
            dp[i][j] = (dp[i-1][j-1]*inv(fact[prime_counts[i]-1]) + dp[i-1][j]*inv(fact[prime_counts[i]]))%MOD;
        }
    }
    out_line!(constant*dp[prime_counts.len()-1][n] % MOD);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = 1;
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
