//{"name":"B. Fedya and Array","group":"Codeforces - Codeforces Round #852 (Div. 2)","url":"https://codeforces.com/contest/1793/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3 -2\n4 -4\n2 -1\n5 -3\n","output":"10\n0 1 2 1 0 -1 0 -1 0 1\n16\n-2 -1 -2 -1 0 1 2 3 4 5 4 3 2 1 0 -1\n6\n1 0 -1 0 1 0\n16\n2 3 2 1 0 -1 0 -1 0 -1 0 1 2 1 0 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFedyaAndArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut x: i64 = input.read();
    let mut y: i64 = input.read();

    if x>y {
        std::mem::swap(&mut x, &mut y);
    }
    out_line!((y-x)*2);
    out_line!((x..y).collect::<Vec<_>>());
    out_line!((x+1..y+1).collect::<Vec<_>>());

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
