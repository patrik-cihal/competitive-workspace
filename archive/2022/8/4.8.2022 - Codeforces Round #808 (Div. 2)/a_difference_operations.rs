//{"name":"A. Difference Operations","group":"Codeforces - Codeforces Round #808 (Div. 2)","url":"https://codeforces.com/problemset/problem/1708/A","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n2\n5 10\n3\n1 2 3\n4\n1 1 1 1\n9\n9 9 8 2 4 4 3 5 3\n","output":"YES\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ADifferenceOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let d: i32 = input.read();
    let mut c = false;
    for i in 1..n {
        let a: i32 = input.read();
        if a%d != 0 {
            c = true;
        }
    }
    if c {
        out_line!("NO");
    }else {
        out_line!("YES");
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
