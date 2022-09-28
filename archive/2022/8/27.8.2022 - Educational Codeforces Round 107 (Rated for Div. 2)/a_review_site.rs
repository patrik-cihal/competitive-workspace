//{"name":"A. Review Site","group":"Codeforces - Educational Codeforces Round 107 (Rated for Div. 2)","url":"https://codeforces.com/contest/1511/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n2\n3\n1 2 3\n5\n1 1 1 1 1\n3\n3 3 2\n","output":"0\n2\n5\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AReviewSite"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut answer = 0;
    let v: Vec<i32> = input.read_vec(n);
    for i in 0..n {
        if v[i] == 1 || v[i] == 3 {
            answer += 1;
        }
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
