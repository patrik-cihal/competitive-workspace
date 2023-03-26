//{"name":"C. Scoring Subsequences","group":"Codeforces - Codeforces Round 856 (Div. 2)","url":"https://codeforces.com/contest/1794/problem/C","interactive":false,"timeLimit":2500,"tests":[{"input":"3\n3\n1 2 3\n2\n1 1\n5\n5 5 5 5 5\n","output":"1 1 2\n1 1\n1 2 3 4 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CScoringSubsequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a: Vec<usize> = input.read_vec(n);

    let mut j = 0;
    let mut answer = vec![];
    for i in 0..a.len() {
        while a[j] < i-j+1 {
            j +=1;
        }
        answer.push(i-j+1);
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
