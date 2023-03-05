//{"name":"D1. Hot Start Up (easy version)","group":"Codeforces - Codeforces Round 854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/problemset/problem/1799/D1","interactive":false,"timeLimit":1000,"tests":[{"input":"9\n3 2\n1 2 2\n3 2\n2 1\n4 2\n1 2 1 2\n5 3\n2 1\n4 3\n1 2 3 1\n100 100 100\n1 1 1\n5 2\n2 1 2 1 1\n65 45\n54 7\n5 3\n1 3 2 1 2\n2 2 2\n1 1 1\n5 1\n1 1 1 1 1\n1000000000\n999999999\n5 6\n1 6 1 4 1\n3 6 4 1 4 5\n1 1 1 1 4 1\n1 3\n3\n4 5 6\n1 2 3\n8 3\n3 3 3 1 2 3 2 1\n10 10 8\n10 10 5\n","output":"6\n11\n301\n225\n8\n4999999996\n11\n6\n63\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1HotStartUpEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, minim};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, k): (usize, usize) = input.read();

    let a: Vec<usize> = input.read_vec::<usize>(n).into_iter().map(|x| x-1).collect();

    let cold: Vec<u64> = input.read_vec(k);
    let hot: Vec<u64> = input.read_vec(k);

    let mut dp = vec![vec![vec![u64::MAX/2; k+1]; 2]; n];

    dp[0][0][k] = cold[a[0]];

    let get_time = |i, j| {
        if i==j {
            hot[i]
        }
        else {
            cold[i]
        }
    };

    for i in 1..n {
        for j in 0..k+1 {
            dp[i][0][j] = dp[i-1][0][j] + get_time(a[i], a[i-1]);
            dp[i][1][j] = dp[i-1][1][j] + get_time(a[i], a[i-1]);
        } 
        dp[i][0][a[i-1]] = dp[i-1][1][a[i]] + hot[a[i]];
        dp[i][1][a[i-1]] = dp[i-1][0][a[i]] + hot[a[i]];
        for j in 0..k+1 {
            minim!(dp[i][0][a[i-1]], dp[i-1][1][j] + cold[a[i]]);
            minim!(dp[i][1][a[i-1]], dp[i-1][0][j] + cold[a[i]]);
        }
    }

    let top_min = *dp[n-1][0].iter().min().unwrap();
    let bot_min = *dp[n-1][1].iter().min().unwrap();

    out_line!(top_min.min(bot_min));
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
