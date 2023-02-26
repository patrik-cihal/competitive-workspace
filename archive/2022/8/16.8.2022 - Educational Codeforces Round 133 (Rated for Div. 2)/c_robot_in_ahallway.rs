//{"name":"C. Robot in a Hallway","group":"Codeforces - Educational Codeforces Round 133 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1716/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 0 1\n4 3 2\n5\n0 4 8 12 16\n2 6 10 14 18\n4\n0 10 10 10\n10 10 10 10\n2\n0 0\n0 0\n","output":"5\n19\n17\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRobotInAHallway"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    
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
