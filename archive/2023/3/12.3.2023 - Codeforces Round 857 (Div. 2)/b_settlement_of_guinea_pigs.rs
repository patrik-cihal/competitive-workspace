//{"name":"B. Settlement of Guinea Pigs","group":"Codeforces - Codeforces Round 857 (Div. 2)","url":"https://codeforces.com/contest/1802/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\n1 1 1\n3\n2 2 2\n5\n1 1 1 2 1\n10\n1 2 1 2 1 2 1 2 1 2\n20\n1 2 1 1 1 1 1 2 1 2 1 2 2 1 1 1 1 1 1 1\n20\n2 1 1 2 1 1 2 1 2 2 1 1 1 2 2 1 1 1 1 2\n","output":"3\n0\n3\n4\n12\n9\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BSettlementOfGuineaPigs"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let v: Vec<u8> = input.read_vec(n);

    let mut stored = 0;
    let mut unstored = 0;

    let mut answer = 0;

    for i in 0..n {
        if v[i] == 1 {
            unstored += 1;
        }
        else {
            if unstored != 0 {
                if unstored %2 == 0 {
                    stored += (unstored-1)/2;
                    unstored = 2;
                }
                else {
                    stored += unstored/2;
                    unstored = 1;
                }
            }
        }
        answer = answer.max(stored+unstored);
    }
    out_line!(answer);
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
