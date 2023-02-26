//{"name":"B. Ideal Point","group":"Codeforces - Educational Codeforces Round 143 (Rated for Div. 2)","url":"https://codeforces.com/contest/1795/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 3\n1 3\n7 9\n2 5\n3 6\n2 9\n1 4\n3 7\n1 3\n2 4\n3 5\n1 4\n6 7\n5 5\n","output":"YES\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BIdealPoint"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();

    let mut start = false;
    let mut end = false;

    for _ in 0..n {
        let l: usize = input.read();
        let r: usize = input.read();

        if l == k {
            start = true;
        }
        if r == k {
            end = true;
        }
    }

    if start && end {
        out_line!("YES");
    } else {
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
