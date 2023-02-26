//{"name":"A. GamingForces","group":"Codeforces - Educational Codeforces Round 142 (Rated for Div. 2)","url":"https://codeforces.com/contest/1792/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n1 2 1 2\n3\n2 4 2\n5\n1 2 3 4 5\n","output":"3\n3\n5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AGamingForces"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);
    let mut count_one = 0;
    for val in v {
        if val == 1 {
            count_one += 1;
        }
    }

    let mut result = 0;
    result += count_one/2;
    result += count_one%2;
    result += n-count_one;

    out_line!(result);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read::<usize>();
    for i in 0..t {

    solve(&mut input, i);
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
