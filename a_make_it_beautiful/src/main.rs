//{"name":"A. Make it Beautiful","group":"Codeforces - Educational Codeforces Round 141 (Rated for Div. 2)","url":"https://codeforces.com/contest/1783/problem/A","interactive":false,"timeLimit":3000,"tests":[{"input":"4\n4\n3 3 6 6\n2\n10 10\n5\n1 2 3 4 5\n3\n1 4 4\n","output":"YES\n3 6 3 6\nNO\nYES\n2 4 1 5 3\nYES\n1 4 4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMakeItBeautiful"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut v = input.read_vec::<i32>(n);

    v.sort();
    v.reverse();
    out_line!(v);
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
