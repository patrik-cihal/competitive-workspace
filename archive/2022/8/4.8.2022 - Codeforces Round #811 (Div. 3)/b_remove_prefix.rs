//{"name":"B. Remove Prefix","group":"Codeforces - Codeforces Round #811 (Div. 3)","url":"https://codeforces.com/contest/1714/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4\n3 1 4 3\n5\n1 1 1 1 1\n1\n1\n6\n6 5 4 3 2 1\n7\n1 2 1 7 1 2 1\n","output":"1\n4\n0\n0\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BRemovePrefix"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v: Vec<usize> = input.read_vec(n);
    let mut seen = vec![false; 200001];
    for i in (0..n).rev() {
        if seen[v[i]] {
            out_line!(i+1);
            return;
        }
        else {
            seen[v[i]] = true;
        }
    }
    out_line!(0);
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
