//{"name":"A. A+B?","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4+2\n0+0\n3+7\n8+9\n","output":"6\n0\n10\n17\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAB"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<String>();
    let mut ex=    s.split('+');

    let a = ex.next().unwrap().parse::<i32>().unwrap();
    let b = ex.next().unwrap().parse::<i32>().unwrap();

    out_line!(a+b);

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
