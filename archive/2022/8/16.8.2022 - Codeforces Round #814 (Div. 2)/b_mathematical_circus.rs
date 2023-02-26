//{"name":"B. Mathematical Circus","group":"Codeforces - Codeforces Round #814 (Div. 2)","url":"https://codeforces.com/contest/1719/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 1\n2 0\n12 10\n14 11\n","output":"YES\n1 2\n3 4\nNO\nYES\n3 4\n7 8\n11 12\n2 1\n6 5\n10 9\nYES\n1 2\n3 4\n5 6\n7 8\n9 10\n11 12\n13 14\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMathematicalCircus"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize =input.read();
    let k: usize = input.read();
    let c = k%4;

    if k%2 == 1{
        out_line!("YES");
        for i in (1..n+1).step_by(2) {
            out_line!(i, i+1);
        }
    }
    else {
        if k%4 == 0 {
            out_line!("NO");
        }
        else {
            out_line!("YES");
            for i in (1..n+1).step_by(2) {
                if i%4 == 1 {
                    out_line!(i+1, i);
                }
                else {
                    out_line!(i, i+1);
                }
            }
        }
    }

    // go from 0
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
