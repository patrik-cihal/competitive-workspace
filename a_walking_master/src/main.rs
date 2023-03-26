//{"name":"A. Walking Master","group":"Codeforces - Codeforces Round 858 (Div. 2)","url":"https://codeforces.com/contest/1806/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n-1 0 -1 2\n0 0 4 5\n-2 -1 1 1\n-3 2 -3 2\n2 -1 -1 -1\n1 1 0 2\n","output":"4\n6\n-1\n0\n3\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AWalkingMaster"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (a, b, c, d): (i64, i64, i64, i64) = input.read();

    let (x, y) = (c-a, d-b);

    if x > y || y<0 {
        out_line!(-1);
    }
    else {
        out_line!(y-x+y);
    }

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
