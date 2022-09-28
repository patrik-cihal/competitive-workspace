//{"name":"B. Beautiful Array","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n1 6 3 100\n3 6 3 12\n3 6 3 19\n5 4 7 38\n5 4 7 80\n99978 1000000000 100000000 1000000000000000000\n1 1 0 0\n4 1000000000 1000000000 1000000000000000000\n","output":"-1\n-1\n0 0 19\n0 3 3 3 29\n-1\n-1\n0\n0 0 0 1000000000000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBeautifulArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();
    let b: usize = input.read();
    let s: usize = input.read();

    if n*(k-1)+k*b < s || b*k>s {
        out_line!(-1);
        return;
    }

    let mut rem = s - k*b;

    let mut v = vec![0; n];

    for i in 0..n {
        let d = rem.min(k-1);
        v[i] = d;
        rem -= d;
    }

    v[0] += b*k;

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
