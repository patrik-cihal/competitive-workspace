//{"name":"A. Greatest Convex","group":"Codeforces - Codeforces Round #842 (Div. 2)","url":"https://codeforces.com/contest/1768/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n6\n8\n10\n","output":"2\n5\n7\n9\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGreatestConvex"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let k: usize = input.read();
    out_line!(k-1);
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
