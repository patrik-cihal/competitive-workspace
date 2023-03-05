//{"name":"D. Maximum Subarray","group":"Codeforces - Educational Codeforces Round 144 (Rated for Div. 2)","url":"https://codeforces.com/contest/1796/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 1 2\n2 -1 2 3\n2 2 3\n-1 2\n3 0 5\n3 2 4\n6 2 -8\n4 -1 9 -3 7 -8\n","output":"5\n7\n0\n44\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaximumSubarray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::minim_maxim::MinimMaxim;
use algo_lib::{out, out_line};


fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let (k, x): (usize, i64) = input.read();
    let a = input.read_vec::<i64>(n);

    let mut dp = vec![vec![0; k+1]; n];

    dp[0][0] = (a[0]-x).max(0);
    if k>0 {
        dp[0][1] = (a[0]+x).max(0);
    }

    for i in 1..n {
        dp[i][0] = (dp[i-1][0] + a[i] - x).max(0);
        for j in 1..(k+1).min(i+1) {
            dp[i][j] = dp[i-1][j] + a[i] - x;
        }

        for j in 1..(k+1).min(i+2) {
            dp[i][j] = dp[i][j].max(dp[i-1][j-1] + a[i] + x).max(0);
        }
    }

    let mut answer = 0;
    for i in 0..n {
        for j in 0..k+1 {
            if n-i-1+j >= k {
                answer.maxim(dp[i][j]);
            }
        }
    }

    out_line!(answer);
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
