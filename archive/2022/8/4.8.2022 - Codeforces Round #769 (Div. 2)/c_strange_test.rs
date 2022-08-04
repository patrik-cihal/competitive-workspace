//{"name":"C. Strange Test","group":"Codeforces - Codeforces Round #769 (Div. 2)","url":"https://codeforces.com/contest/1632/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n1 3\n5 8\n2 5\n3 19\n56678 164422\n","output":"1\n3\n2\n1\n23329\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CStrangeTest"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a: i32 = input.read();
    let b: i32 = input.read();

    let mut result = b-a;
    for i in 0..b-a {
        let candidate = (a|(b+i))-(b+i);
        result = result.min(candidate+i+1);
    }
    out_line!(result);
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
