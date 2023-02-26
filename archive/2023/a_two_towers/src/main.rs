//{"name":"A. Two Towers","group":"Codeforces - Educational Codeforces Round 143 (Rated for Div. 2)","url":"https://codeforces.com/contest/1795/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 3\nBRBB\nRBR\n4 7\nBRBR\nRRBRBRB\n3 4\nRBR\nBRBR\n5 4\nBRBRR\nBRBR\n","output":"YES\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATwoTowers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut s: Vec<char> = input.read::<String>().chars().collect();
    let mut t: Vec<char> = input.read::<String>().chars().collect();
    t.reverse();

    s.extend(t);

    let mut found = false;
    for i in 1..s.len() {
        if s[i] == s[i-1] {
            if found {
                out_line!("NO");
                return;
            }
            found = true;
        }
    }

    out_line!("YES");

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
