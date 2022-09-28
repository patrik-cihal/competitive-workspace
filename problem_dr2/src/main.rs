//{"name":"Problem D - R2","group":"Kattis - UP_PA","url":"https://open.kattis.com/contests/z9oe6v/problems/r2","interactive":false,"timeLimit":1000,"tests":[{"input":"11 15\n","output":"19\n"},{"input":"4 3\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ProblemDR2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let r1: i32 = input.read();
    let s: i32 = input.read();
    let r2 = s*2 - r1;
    out_line!(r2);
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
