//{"name":"A. Cowardly Rooks","group":"Codeforces - Educational Codeforces Round 138 (Rated for Div. 2)","url":"https://codeforces.com/contest/1749/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2 2\n1 2\n2 1\n3 1\n2 2\n","output":"NO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ACowardlyRooks"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    for i in 0..m {
        input.read::<usize>();
        input.read::<usize>();
    }

    if m < n {
        out_line!("YES");

    }
    else {
        out_line!("NO");
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
