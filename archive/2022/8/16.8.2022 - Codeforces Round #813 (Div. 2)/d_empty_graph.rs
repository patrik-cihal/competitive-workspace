//{"name":"D. Empty Graph","group":"Codeforces - Codeforces Round #813 (Div. 2)","url":"https://codeforces.com/contest/1712/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"6\n3 1\n2 4 1\n3 2\n1 9 84\n3 1\n10 2 6\n3 2\n179 17 1000000000\n2 1\n5 9\n2 2\n4 2\n","output":"4\n168\n10\n1000000000\n9\n1000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DEmptyGraph"}}}

use std::mem::swap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::collections::segment_tree::SegmentTree;

const MAX: i32 = 1000000000;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();

    let v = input.read_vec::<i32>(n);

    let mut sv = v.clone();
    sv.sort();

    if k==n {
        out_line!(MAX);
        return;
    }

    let mut ans = 0;
    if k != 1 {
        for i in 1..n {
            if (v[i] <= sv[k-1] && v[i-1] <= sv[k-2]) || (v[i] <= sv[k-2] && v[i-1] <= sv[k-1]) {
                ans = sv[k]*2;
                out_line!(ans.min(MAX));
                return;
            }
        }
    }


    for i in 1..n {
        let mut a = v[i];
        let mut b=  v[i-1];
        // check whether a or b should be raised to 10^9
        // seperate logic for k = 1
        if k==1 {
            if a==sv[0] && b==sv[0] {
                ans = ans.max(a.max(b).min(sv[1]*2));
            }
            else {
                if a==sv[0] {
                    a = MAX;
                }
                if b == sv[0] {
                    b = MAX;
                }
                ans = ans.max(a.min(b).min(sv[1]*2));
            }

        }
        // check whether we have overcount
        else if a==sv[k-1] && b==sv[k-1] && sv[k-1] != sv[k-2] {
            ans = ans.max(a.max(b).min((sv[k]*2)));
        }
        // for rest apply standard logic
        else {
            if a<=sv[k-1] {
                a = MAX;
            }
            if b <= sv[k-1] {
                b = MAX;
            }
            ans = ans.max(a.min(b).min((sv[k]*2)));
        }
    }


    out_line!(ans);
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
