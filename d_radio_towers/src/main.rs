//{"name":"D. Radio Towers","group":"Codeforces - Educational Codeforces Round 98 (Rated for Div. 2)","url":"https://codeforces.com/contest/1452/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n","output":"748683265\n"},{"input":"3\n","output":"748683265\n"},{"input":"5\n","output":"842268673\n"},{"input":"200000\n","output":"202370013\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRadioTowers"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::modulo::mod_inv;
use algo_lib::{out, out_line};

const MOD: u64 = 998244353;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a = 1;
    let mut b = 1; 

    for i in 1..n {
        let b_copy = b;
        b = (a+b) % MOD;
        a = b_copy;
    }

    let mut den = 1;

    for i in 0..n {
        den *= 2;
        den %= MOD;
    }

    out_line!((a*mod_inv(den, MOD))%MOD);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
