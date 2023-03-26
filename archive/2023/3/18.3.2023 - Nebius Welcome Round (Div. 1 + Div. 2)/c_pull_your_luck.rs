//{"name":"C. Pull Your Luck","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/C?478d8f0e03ca8d4f1e031ab2aea877cf6bffdf18fbadd002b719e0a9d183d8a8=1","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n5 2 1\n5 2 2\n10 0 100\n11 7 100\n3 1 1000\n31 0 10\n100 49 7\n","output":"No\nYes\nYes\nYes\nNo\nNo\nNo\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPullYourLuck"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, x, p): (usize, usize, usize) = input.read();

    let mut sum = x;
    for i in 1..(n*2).min(p+1) {
        sum = (sum+i)%(n);
        if sum == 0 {
            out_line!("YES");
            return;
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
