//{"name":"B. Build the Permutation","group":"Codeforces - Codeforces Round #758 (Div.1 + Div. 2)","url":"https://codeforces.com/contest/1608/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4 1 1\n6 1 2\n6 4 0\n","output":"1 3 2 4\n4 2 3 1 5 6\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BBuildThePermutation"}}}

use std::mem::swap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut a: usize = input.read();
    let mut b: usize = input.read();


    let c = a>b;

    if a>b {
        swap(&mut a, &mut b);
    }

    if b-a > 1 {
        out_line!(-1);
        return;
    }

    let mut answer = vec![0; n];
    if b>a {
        if b>(n-1)/2 {
            out_line!(-1);
            return;
        }
        for i in 0..b {
            answer[i*2] = i+1;
            answer[i*2+1] = n-i;
        }
        for i in b*2..n {
            answer[i] = n-(i-b);
        }
    }
    else {
        if b>(n-2)/2 {
            out_line!(-1);
            return;
        }
        for i in 0..b {
            answer[i*2] = i+1;
            answer[i*2+1] = n-i;
        }
        for i in b*2..n {
            answer[i] = (i-b)+1;
        }
    }

    if !c {
        for i in 0..n {
            answer[i] = n-answer[i]+1;
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
