//{"name":"Problem A - Solving for Carrots","group":"Kattis - UP_PA","url":"https://open.kattis.com/contests/z9oe6v/problems/carrots","interactive":false,"timeLimit":1000,"tests":[{"input":"2 1\ncarrots?\nbunnies\n","output":"1\n"},{"input":"1 5\nsovl problmz\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ProblemASolvingForCarrots"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let p: usize = input.read();
    out_line!(p);
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
