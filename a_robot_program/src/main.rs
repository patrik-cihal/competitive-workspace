//{"name":"A. Robot Program","group":"Codeforces - Educational Codeforces Round 98 (Rated for Div. 2)","url":"https://codeforces.com/contest/1452/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 5\n3 4\n7 1\n0 0\n2 0\n","output":"10\n7\n13\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARobotProgram"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    
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
