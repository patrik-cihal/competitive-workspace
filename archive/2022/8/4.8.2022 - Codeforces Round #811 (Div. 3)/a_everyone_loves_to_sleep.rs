//{"name":"A. Everyone Loves to Sleep","group":"Codeforces - Codeforces Round #811 (Div. 3)","url":"https://codeforces.com/contest/1714/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 6 13\n8 0\n3 6 0\n12 30\n14 45\n6 0\n2 23 35\n20 15\n10 30\n","output":"1 47\n0 0\n10 55\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AEveryoneLovesToSleep"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};


fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let st = input.read::<i32>()*60+input.read::<i32>();
    let mut mt = 24*60;
    for i in 0..n {
        let t = input.read::<i32>()*60+input.read::<i32>();
        if t<st {
            mt = mt.min(24*60-st+t);
        }else {
            mt = mt.min(t-st);
        }
    }
    out_line!(mt/60, mt%60);
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
