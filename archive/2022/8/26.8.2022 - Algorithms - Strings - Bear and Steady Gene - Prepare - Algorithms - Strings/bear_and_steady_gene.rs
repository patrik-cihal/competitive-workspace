//{"name":"Bear and Steady Gene","group":"HackerRank - Algorithms - Strings - Bear and Steady Gene - Prepare - Algorithms - Strings","url":"https://www.hackerrank.com/challenges/bear-and-steady-gene/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"8\nGAAATAAA\n","output":"5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BearAndSteadyGene"}}}

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
