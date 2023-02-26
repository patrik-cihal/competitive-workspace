//{"name":"B. Permutation Value","group":"Codeforces - Educational Codeforces Round 137 (Rated for Div. 2)","url":"https://codeforces.com/contest/1743/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n5\n6\n","output":"1 4 3 5 2\n4 1 6 2 5 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPermutationValue"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    out!(1, "");
    for i in 3..n+1 {
        out!(i, "");
    }
    out_line!(2);
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
