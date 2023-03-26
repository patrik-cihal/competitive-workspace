//{"name":"B. Not Dividing","group":"Codeforces - Codeforces Round 856 (Div. 2)","url":"https://codeforces.com/contest/1794/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n2 4 3 6\n3\n1 2 3\n2\n4 2\n","output":"4 5 6 7\n3 2 3\n4 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNotDividing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut v: Vec<u32> = input.read_vec(n);

    for i in 0..n-1 {
        if v[i] == 1 {
            v[i] += 1;
        }
    }
    for i in 1..n {
        if v[i]%v[i-1] == 0 {
            v[i] += 1;
        }
    }
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
