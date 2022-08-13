//{"name":"The Coin Change Problem","group":"HackerRank - Algorithms - Dynamic Programming - The Coin Change Problem - Prepare - Algorithms - Dynamic Programming","url":"https://www.hackerrank.com/challenges/coin-change/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"4 3\n1 2 3\n","output":"4\n"},{"input":"10 4\n2 5 3 6\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"TheCoinChangeProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut c: Vec<usize> = input.read_vec(m);
    c.sort();

    let mut dp = vec![vec![0; m]; n+1];
    for i in 0..m {
        dp[0][i] = 1;
    }
    for i in 0..n+1 {
        for j in 0..m {
            if i+c[j] > n {
                break;
            }
            for k in j..m {
                dp[i+c[j]][k] += dp[i][j];
            }
        }
    }
    let ans: usize = dp[n][m-1];
    out_line!(ans);
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
