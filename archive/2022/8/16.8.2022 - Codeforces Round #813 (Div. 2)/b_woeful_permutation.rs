//{"name":"B. Woeful Permutation","group":"Codeforces - Codeforces Round #813 (Div. 2)","url":"https://codeforces.com/contest/1712/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n2\n","output":"1\n2 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BWoefulPermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut answer = vec![0; n];
    for i in 0..n {
        answer[i] = i+1;
    }
    for i in (n%2..n).step_by(2) {
        answer.swap(i, i+1)
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
