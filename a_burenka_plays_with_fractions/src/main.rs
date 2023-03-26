//{"name":"A. Burenka Plays with Fractions","group":"Codeforces - Codeforces Round 815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n2 1 1 1\n6 3 2 1\n1 2 2 3\n0 1 0 100\n0 1 228 179\n100 3 25 6\n999999999 300000000 666666666 100000000\n33 15 0 84\n","output":"1\n0\n2\n0\n1\n1\n1\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ABurenkaPlaysWithFractions"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (a, b, c, d): (usize, usize, usize, usize) = input.read();

    if a*d ==c*b {
        out_line!(0);
    }
    else if (c*b != 0 &&(a*d) % (c*b) == 0) || (a*d != 0 && (c*b) % (a*d) == 0) {
        out_line!(1);
    }
    else {
        out_line!(2);
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
