//{"name":"D1. Xor-Subsequence (easy version)","group":"Codeforces - Codeforces Round 815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n1 2\n5\n5 2 4 3 1\n10\n3 8 8 2 9 1 6 2 8 3\n","output":"2\n3\n6\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1XorSubsequenceEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, maxim};

const K: usize = 256;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let a = input.read_vec::<usize>(n);


    let mut count = vec![1; n];

    for cur in (0..n).step_by(K) {
        for i in cur..(cur+K).min(n) {
            for j in i+1..(cur+K).min(n) {
                if a[i]^j < a[j]^i {
                    maxim!(count[j], count[i]+1);
                }
            }
        }
    }
    let answer = *count.iter().max().unwrap();
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
