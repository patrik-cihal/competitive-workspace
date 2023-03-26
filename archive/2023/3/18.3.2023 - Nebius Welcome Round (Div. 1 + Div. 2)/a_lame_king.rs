//{"name":"A. Lame King","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n-4 1\n4 4\n0 -6\n-5 -4\n7 -8\n","output":"7\n8\n11\n9\n15\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALameKing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (a, b): (i32, i32) = input.read();

    let mut a = a.abs();
    let mut b = b.abs();

    if a>b {
        std::mem::swap(&mut a, &mut b);
    }

    if a==b {
        out_line!(a+b);
        return;
    }

    let offset = b-a;

    let mut answer = a*2;

    let mut additional = (offset-1) * 2 + 1;
    out_line!(answer + additional);
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
