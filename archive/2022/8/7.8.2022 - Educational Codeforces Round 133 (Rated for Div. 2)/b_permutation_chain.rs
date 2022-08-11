//{"name":"B. Permutation Chain","group":"Codeforces - Educational Codeforces Round 133 (Rated for Div. 2)","url":"https://codeforces.com/contest/1716/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n2\n3\n","output":"2\n1 2\n2 1\n3\n1 2 3\n3 2 1\n3 1 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPermutationChain"}}}

use std::mem::swap;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    out_line!(n);
    let mut v = vec![0; n];
    for i in 0..n {
        v[i] = i+1;
    }
    for i in 1..n {
        out_line!(v);
        let t = v[i];
        v[i] = v[i-1];
        v[i-1] = t;
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
