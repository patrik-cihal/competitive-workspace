//{"name":"A. Perfect Permutation","group":"Codeforces - Codeforces Round #810 (Div. 2)","url":"https://codeforces.com/problemset/problem/1711/A","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1\n4\n","output":"1\n2 1 4 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APerfectPermutation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n:usize = input.read();
    out!(n, " ");
    for i in 1..n {
        out!(i, " ");
    }
    out_line!();
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
