//{"name":"Problem E - Nasty Hacks","group":"Kattis - UP_PA","url":"https://open.kattis.com/contests/z9oe6v/problems/nastyhacks","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n0 100 70\n100 130 30\n-100 -70 40\n","output":"advertise\ndoes not matter\ndo not advertise\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ProblemENastyHacks"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let r: i32 = input.read();
    let e: i32 = input.read();
    let c: i32 = input.read();

    if e-c > r {
        out_line!("advertise");
    }
    else if e-c == r {
        out_line!("does not matter");
    }
    else {
        out_line!("do not advertise");
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
