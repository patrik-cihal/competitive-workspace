//{"name":"A. Parallel Projection","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, based on VK Cup 2022 - Elimination Round)","url":"https://codeforces.com/contest/1782/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n55 20 29\n23 10 18 3\n20 10 5\n1 5 2 5\n15 15 4\n7 13 10 10\n2 1000 2\n1 1 1 999\n10 4 10\n7 1 2 1\n","output":"47\n8\n14\n1002\n17\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AParallelProjection"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (w, d, h): (i64, i64, i64) = (input.read(), input.read(), input.read());
    let (a, b, f, g): (i64, i64, i64, i64) = (input.read(), input.read(), input.read(), input.read());


    let candidates = [
        (b-g).abs() + a + f,
        (b-g).abs() + w-a + w-f,
        (a-f).abs() + b + g,
        (a-f).abs() + d-b + d-g,
    ];

    out_line!(candidates.iter().min().unwrap() + h);

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
