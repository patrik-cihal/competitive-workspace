//{"name":"D. Min Cost String","group":"Codeforces - Educational Codeforces Round 107 (Rated for Div. 2)","url":"https://codeforces.com/contest/1511/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"9 4\n","output":"aabacadbb\n"},{"input":"5 1\n","output":"aaaaa\n"},{"input":"10 26\n","output":"codeforces\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMinCostString"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: u32 = input.read();

    let mut answer = vec![];

    for i in 0..k-1 {
        answer.push(i);
        for j in (i+2..k).rev() {
            answer.push(j);
            answer.push(i);
        }
    }
    for i in (0..k).rev() {
        answer.push(i);
        answer.push(i);
    }
    answer.pop();
    for i in 0..n {
        out!(char::from_u32('a' as u32 + answer[i%answer.len()]).unwrap());
    }
    out_line!();
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
