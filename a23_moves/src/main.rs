//{"name":"A. 2-3 Moves","group":"Codeforces - Educational Codeforces Round 133 (Rated for Div. 2)","url":"https://codeforces.com/contest/1716/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n3\n4\n12\n","output":"2\n1\n2\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A23Moves"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: i32 = input.read();
    if n == 1 {
        out_line!(2);
    }else {
        out_line!((n+2)/3);
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
