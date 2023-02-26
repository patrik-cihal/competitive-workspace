//{"name":"Problem C - N-sum","group":"Kattis - UP_PA","url":"https://open.kattis.com/contests/z9oe6v/problems/nsum","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n1 1\n","output":"2\n"},{"input":"5\n1 2 3 4 5\n","output":"15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ProblemCNSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut answer = 0;
    let n: usize = input.read();
    for i in 0..n {
        answer += input.read::<i32>();
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
