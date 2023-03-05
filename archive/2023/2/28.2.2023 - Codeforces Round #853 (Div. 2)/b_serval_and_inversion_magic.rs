//{"name":"B. Serval and Inversion Magic","group":"Codeforces - Codeforces Round #853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n4\n1001\n5\n10010\n7\n0111011\n","output":"Yes\nYes\nNo\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BServalAndInversionMagic"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let a: Vec<usize> = input.read::<String>().chars().map(|c| c.to_digit(10).unwrap() as usize).collect();

    let mut continuous = false;
    let mut found = false;

    for i in 0..n/2 {
        if a[i] != a[n-i-1] {
            if !found {
                found = true;
                continuous = true;
            }
            else {
                if continuous {
                    continue;
                }
                else {
                    out_line!("No");
                    return;
                }
            }
        }
        else {
            continuous = false;
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
