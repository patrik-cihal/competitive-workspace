//{"name":"B. GCD Length","group":"Codeforces - Educational Codeforces Round 107 (Rated for Div. 2)","url":"https://codeforces.com/contest/1511/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2 3 1\n2 2 2\n6 6 2\n1 1 1\n","output":"11 492\n13 26\n140133 160776\n1 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BGCDLength"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a: i32 = input.read();
    let b: i32 = input.read();
    let c: i32 = input.read();

    let mut z = 1;
    for i in 1..c {
        z *= 10;
        z += 1;
    }
    let mut bm  = 1;
    for i in 0..b-c {
        bm*=10;
        bm += 1;
    }
    let y = z*bm;
    let x = z*10i32.pow((a-c) as u32);
    out_line!(x, y);
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
