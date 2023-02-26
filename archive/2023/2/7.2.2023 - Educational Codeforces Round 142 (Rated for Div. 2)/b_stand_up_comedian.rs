//{"name":"B. Stand-up Comedian","group":"Codeforces - Educational Codeforces Round 142 (Rated for Div. 2)","url":"https://codeforces.com/contest/1792/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 0 0 0\n0 0 0 5\n2 5 10 6\n3 0 0 7\n","output":"5\n1\n15\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BStandUpComedian"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let v = input.read_vec::<i64>(4);

    let sum = v.iter().sum::<i64>();

    if v[0] == 0 && sum != 0 {
        out_line!(1);
        return;
    }

    let result = v[0] + v[1].min(v[2]) * 2 + v[0].min((v[1]-v[2]).abs() + v[3]);

    if v[0] >= (v[1]-v[2]).abs() + v[3] {
        out_line!(result);
    }
    else {
        out_line!(result + 1);
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
