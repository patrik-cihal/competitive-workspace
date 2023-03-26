//{"name":"A. Fingerprints","group":"Codeforces - Codeforces Round 488 by NEAR (Div. 2)","url":"https://codeforces.com/contest/994/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7 3\n3 5 7 1 6 2 8\n1 2 7\n","output":"7 1 2\n"},{"input":"4 4\n3 4 1 0\n0 1 7 9\n","output":"1 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFingerprints"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();

    let a = input.read_vec::<usize>(n);
    let b = input.read_vec::<usize>(m);

    let mut fing = vec![false; 10];

    for i in 0..b.len() {
        fing[b[i]] = true;
    }

    let mut answer = vec![];

    for i in 0..n {
        if fing[a[i]] {
            answer.push(a[i]);
        }
    }

    out_line!(answer);

}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
