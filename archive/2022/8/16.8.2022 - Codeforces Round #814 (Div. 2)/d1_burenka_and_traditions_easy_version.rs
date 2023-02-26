//{"name":"D1. Burenka and Traditions (easy version)","group":"Codeforces - Codeforces Round #814 (Div. 2)","url":"https://codeforces.com/contest/1719/problem/D1","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n4\n5 5 5 5\n3\n1 3 2\n2\n0 0\n3\n2 5 7\n6\n1 2 3 3 2 1\n10\n27 27 34 32 2 31 23 56 52 4\n5\n1822 1799 57 23 55\n","output":"2\n2\n0\n2\n4\n7\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1BurenkaAndTraditionsEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);

    // make dp of length 0 to n
    let mut dp = vec![vec![0; n]; n];

    let mut k = 1;
    let mut result = 0;

    if v[0] == 0 {
        result += 1;
    }
    dp[0][0] = v[0];

    // go from 0 to n
    for i in 1..n {
        // at each iteration go from left pointer to i and update the xor possible
        // if one is zero, update left pointer and add to result
        dp[i][0] = v[i];
        for j in 0..k {
            dp[i][j+1] = v[i] ^ dp[i-1][j];
        }
        k += 1;
        let mut c = false;
        for j in 0..k {
            if dp[i][j] == 0 {
                c = true;
                break;
            }
        }
        if c {
            result += 1;
            dp[i][0] = 0;
            k = 1;
        }
    }
    // the answer is n-result
    out_line!(n-result);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
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
