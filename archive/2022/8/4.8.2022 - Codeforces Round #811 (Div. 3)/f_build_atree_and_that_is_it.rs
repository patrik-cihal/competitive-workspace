//{"name":"F. Build a Tree and That Is It","group":"Codeforces - Codeforces Round #811 (Div. 3)","url":"https://codeforces.com/contest/1714/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"9\n5 1 2 1\n5 2 2 2\n5 2 2 3\n5 2 2 4\n5 3 2 3\n4 2 1 1\n4 3 1 1\n4 1 2 3\n7 1 4 1\n","output":"YES\n1 2\n4 1\n3 1\n2 5\nYES\n4 3\n2 5\n1 5\n5 3\nNO\nYES\n2 4\n4 1\n2 5\n5 3\nYES\n5 4\n4 1\n2 5\n3 5\nYES\n2 3\n3 4\n1 3\nNO\nYES\n4 3\n1 2\n2 4\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBuildATreeAndThatIsIt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let d12:usize = input.read();
    let d23: usize = input.read();
    let d13: usize = input.read();

    let m = d12.min(d23).min(d13);
    if d12 == m {
        if d23 + d13 != n+1  && d23.max(d13) - d13.min(d23) > d12 {
            out_line!(_test_case, "NO");
            return;
        }
    }
    else if d23 == m {
        if d12 + d13 != n+1  && d12.max(d13) - d13.min(d12) > d23 {
            out_line!(_test_case, "NO");
            return;
        }
    }
    else if d13 == m {
        if d23 + d12 != n+1  && d23.max(d12) - d12.min(d23) > d13 {
            out_line!(_test_case, "NO");
            return;
        }
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
