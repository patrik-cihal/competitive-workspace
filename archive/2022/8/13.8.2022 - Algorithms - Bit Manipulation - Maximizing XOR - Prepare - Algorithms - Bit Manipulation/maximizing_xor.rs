//{"name":"Maximizing XOR","group":"HackerRank - Algorithms - Bit Manipulation - Maximizing XOR - Prepare - Algorithms - Bit Manipulation","url":"https://www.hackerrank.com/challenges/maximizing-xor/problem?isFullScreen=true&h_r=next-challenge&h_v=zen","interactive":false,"timeLimit":4000,"tests":[{"input":"10\n15\n","output":"7\n"},{"input":"11\n100\n","output":"127\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"MaximizingXOR"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut ans = 0;
    let l: usize = input.read();
    let r: usize = input.read();
    for i in l..r+1 {
        for j in l..r+1 {
            ans = ans.max(i^j);
        }
    }   
    out_line!(ans);
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
