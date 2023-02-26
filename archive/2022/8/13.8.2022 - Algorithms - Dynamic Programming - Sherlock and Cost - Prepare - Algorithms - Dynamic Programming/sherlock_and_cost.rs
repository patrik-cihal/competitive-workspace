//{"name":"Sherlock and Cost","group":"HackerRank - Algorithms - Dynamic Programming - Sherlock and Cost - Prepare - Algorithms - Dynamic Programming","url":"https://www.hackerrank.com/challenges/sherlock-and-cost/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"1\n5\n10 1 10 1 10\n","output":"36\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SherlockAndCost"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let b: Vec<i32> = input.read_vec::<i32>(n).into_iter().map(|x| x-1).collect();

    let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; n];

    for i in 1..n {
        dp[i][0] = (b[i-1]+dp[i-1][1]).max(dp[i-1][0]);
        dp[i][1] = ((b[i]-b[i-1]).abs()+dp[i-1][1]).max(b[i]+dp[i-1][0]);
    }

    out_line!(dp[n-1][0].max(dp[n-1][1]));
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
