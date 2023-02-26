//{"name":"D. Maximum AND","group":"Codeforces - Educational Codeforces Round 134 (Rated for Div. 2)","url":"https://codeforces.com/contest/1721/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n5\n1 0 0 3 3\n2 3 2 1 0\n3\n1 1 1\n0 0 3\n8\n0 1 2 3 4 5 6 7\n7 6 5 4 3 2 1 0\n","output":"2\n0\n7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaximumAND"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a = input.read_vec::<i32>(n);
    let mut b = input.read_vec::<i32>(n);

    let mut answer = 0;
    for i in (0..30).rev() {
        let candidate = answer | (1 << i);
        let mut na = a.iter().map(|x| (*x) & candidate).collect::<Vec<i32>>();
        na.sort();
        let mut nb: Vec<i32> = b.iter().map(|x| !(*x) & candidate).collect();
        nb.sort();
        if nb == na {
            answer = candidate;
        }
    }
    out_line!(answer);
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
