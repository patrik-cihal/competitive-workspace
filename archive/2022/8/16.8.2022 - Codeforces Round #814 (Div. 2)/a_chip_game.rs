//{"name":"A. Chip Game","group":"Codeforces - Codeforces Round #814 (Div. 2)","url":"https://codeforces.com/contest/1719/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1 1\n1 4\n5 6\n2 2\n6 3\n999999999 1000000000\n","output":"Tonya\nBurenka\nBurenka\nTonya\nBurenka\nBurenka\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AChipGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let c = n%2+m%2;
    if c==2 {
        out_line!("Tonya");;
    }
    else if c==1 {
        out_line!("Burenka");
    }
    else {
        out_line!("Tonya");
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
