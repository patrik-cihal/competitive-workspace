//{"name":"E. Add Modulo 10","group":"Codeforces - Codeforces Round #811 (Div. 3)","url":"https://codeforces.com/contest/1714/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n2\n6 11\n3\n2 18 22\n5\n5 10 5 10 5\n4\n1 2 4 8\n2\n4 5\n3\n93 96 102\n2\n40 6\n2\n50 30\n2\n22 44\n2\n1 5\n","output":"Yes\nNo\nYes\nYes\nNo\nYes\nNo\nNo\nYes\nNo\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EAddModulo10"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut v: Vec<i32> = input.read_vec(n);
    let mut cat: Vec<(bool)> = vec![false; n];
    let mut five = false;
    for i in 0..n {
        if v[i]%10 == 0 || v[i] % 10 == 5 {
            five = true;
        }
    }
    if five {
        let t = v[0]+v[0]%10;
        for i in 0..n {
            if v[i]+v[i]%10 != t {
                out_line!("No");
                return;
            }
        }
        out_line!("Yes");
        return;
    }
    for i in 0..n {
        if v[i]%2 == 0 {
            if v[i]%10 == 6 {
                cat[i] = (v[i]/10)%2 == 1;
            }else {
                cat[i] = (v[i]/10)%2 == 0;
            }
        }
        else {
            if v[i]%10 == 1 {
                cat[i] = (v[i]/10)%2 == 0;
            }else {
                cat[i] = (v[i]/10)%2 == 1;
            }
        }
    }
    for i in 1..n {
        if cat[i] != cat[i-1] {
            out_line!("No");
            return;
        }
    }
    out_line!("Yes");
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
