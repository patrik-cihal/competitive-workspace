//{"name":"C. Save the Magazines","group":"Codeforces - Educational Codeforces Round 137 (Rated for Div. 2)","url":"https://codeforces.com/contest/1743/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\n01110\n10 5 8 9 6\n6\n011011\n20 10 9 30 20 19\n4\n0000\n100 100 100 100\n4\n0111\n5 4 5 1\n","output":"27\n80\n0\n14\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSaveTheMagazines"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut s: Vec<char> = input.read::<String>().chars().collect();
    let a = input.read_vec::<i32>(n);

    let mut result = 0;
    let mut i = 0;
    while i<n {
        i += 1;
        // find 1
        // look at previous zero value
        // until 1s value is bigger continue, otherwise change result
        if i==n {
            break;
        }
        if s[i] == '1' && s[i-1] == '0' {
            let prev = a[i-1];
            let j = i-1;
            while i<n && s[i] == '1' {
                if a[i]<prev {
                    s[i] = '0';
                    s[j] = '1';
                    break;
                }
                i += 1;
            }
        }
    }
    for i in 0..n {
        if s[i] == '1' {
            result += a[i];
        }
    }
    out_line!(result);
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
