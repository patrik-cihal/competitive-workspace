//{"name":"C. Ice and Fire","group":"Codeforces - Polynomial Round 2022 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1774/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n001\n4\n101\n","output":"1 1 3\n1 2 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIceAndFire"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let s = input.read::<String>().chars().collect::<Vec<char>>();


    let c = s[0];
    let mut f = false;

    for i in 0..n-1 {
        if !f && s[i] == c {
            out!(1, "");
        }
        else {
            f = true;
            out!(i+1, "");
        }
    }
    out_line!();


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
