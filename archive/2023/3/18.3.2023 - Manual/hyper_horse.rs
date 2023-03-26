//{"name":"hyper_horse","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"hyper_horse"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const MOD: u64 = 1_000_000_007;

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();

    let word = input.read::<String>().chars().collect::<Vec<char>>();

    let w = word.len();

    let table = vec![0; n].into_iter().map(|_| input.read::<String>().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut dp = vec![vec![0; m]; n];

    for i in 0..n {
        for j in 0..m {
            if table[i][j] == word[0] {
                dp[i][j] = 1;
            }
        }
    }

    for l in 1..w {
        let mut sum = 0;
        for i in 0..n {
            for j in 0..m {
                sum += dp[i][j];
                sum %= MOD;
            }
        }
        let mut new_dp = vec![vec![0; m]; n];
        let mut rows = vec![0; n];
        for i in 0..n {
            for j in 0..m {
                rows[i] += dp[i][j];
                rows[i] %= MOD;
            }
        }
        let mut columns = vec![0; m];
        for j in 0..m {
            for i in 0..n {
                columns[j] += dp[i][j];
                columns[j] %= MOD;
            }
        }
        let mut diagonals_right = vec![0; n+m-1];
        for i in 0..n {
            for j in 0..m {
                diagonals_right[i+j] += dp[i][j];
                diagonals_right[i+j] %= MOD;
            }
        }

        let mut diagonals_left = vec![0; n+m-1];
        for i in 0..n {
            for j in 0..m {
                diagonals_left[n-i-1+j] += dp[i][j];
                diagonals_left[n-i-1+j] %= MOD;
            }
        }

        for i in 0..n {
            for j in 0..m {
                if table[i][j] == word[l] {
                    new_dp[i][j] = sum+MOD*10 - rows[i] - columns[j] - diagonals_right[i+j] - diagonals_left[n-i-1+j];
                    if word[l] == word[l-1] {
                        new_dp[i][j] += dp[i][j]*3;
                    }
                    new_dp[i][j] %= MOD;
                }
            }
        }
        dp = new_dp;
    }

    let mut answer = 0;

    for i in 0..n {
        for j in 0..m {
            answer += dp[i][j];
            answer %= MOD;
        }
    }

    out_line!(answer);

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
