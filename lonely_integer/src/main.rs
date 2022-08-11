//{"name":"Lonely Integer","group":"HackerRank - Algorithms - Bit Manipulation - Lonely Integer - Prepare - Algorithms - Bit Manipulation","url":"https://www.hackerrank.com/challenges/lonely-integer/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"1\n1\n","output":"1\n"},{"input":"3\n1 1 2\n","output":"2\n"},{"input":"5\n0 0 1 2 1\n","output":"2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"LonelyInteger"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let v = input.read_vec::<i32>(n);
    let mut xsum = 0;
    for i in 0..n {
        xsum ^= v[i];
    }
    out_line!(xsum);
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
