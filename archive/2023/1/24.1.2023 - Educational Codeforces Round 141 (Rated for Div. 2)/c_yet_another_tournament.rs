//{"name":"C. Yet Another Tournament","group":"Codeforces - Educational Codeforces Round 141 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1783/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 401\n100 100 200 1\n3 2\n1 2 3\n5 0\n1 1 1 1 1\n4 0\n0 1 1 1\n4 4\n1 2 2 1\n","output":"1\n2\n6\n4\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CYetAnotherTournament"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    out_line!(n);
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
