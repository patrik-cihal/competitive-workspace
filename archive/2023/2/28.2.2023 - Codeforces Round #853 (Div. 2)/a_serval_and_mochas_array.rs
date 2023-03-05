//{"name":"A. Serval and Mocha's Array","group":"Codeforces - Codeforces Round #853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n2\n3 6\n3\n1 2 4\n3\n3 6 1\n3\n15 35 21\n4\n35 10 35 14\n5\n1261 227821 143 4171 1941\n","output":"No\nYes\nYes\nNo\nYes\nYes\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AServalAndMochasArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::math::gcd;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    

    let a: Vec<usize> = input.read_vec(n);

    for i in 0..n {
        for j in i+1..n {
            if gcd(a[i], a[j]) <= 2 {
                out_line!("YES");
                return;
            }
        }
    }
    out_line!("NO");
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
