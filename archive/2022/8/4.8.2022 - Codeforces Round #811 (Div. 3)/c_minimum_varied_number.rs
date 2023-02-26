//{"name":"C. Minimum Varied Number","group":"Codeforces - Codeforces Round #811 (Div. 3)","url":"https://codeforces.com/contest/1714/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n20\n8\n45\n10\n","output":"389\n8\n123456789\n19\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinimumVariedNumber"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut s: i32 = input.read();
    let mut d: i32 = 9;
    let mut v = vec![];
    for d in (1..10).rev() {
        if d<=s {
            s-=d;
            v.push(d);
        }
    }
    for i in (0..v.len()).rev() {
        out!(v[i]);
    }
    out_line!();
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
