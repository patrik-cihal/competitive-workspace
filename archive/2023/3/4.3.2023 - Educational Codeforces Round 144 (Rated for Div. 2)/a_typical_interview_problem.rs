//{"name":"A. Typical Interview Problem","group":"Codeforces - Educational Codeforces Round 144 (Rated for Div. 2)","url":"https://codeforces.com/contest/1796/problem/0","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3\nFFB\n8\nBFFBFFBF\n3\nBBB\n","output":"YES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATypicalInterviewProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let s = input.read::<String>().chars().collect::<Vec<char>>();

    for i in 1..30 {
        let mut k = Vec::new();
        let mut j = 0;
        while k.len() <= s.len() {
            if (i+j)%3 == 0 {
                k.push('F');
            }

            if (i+j)%5 == 0 {
                k.push('B');
            }
            j += 1;
        }
        if k[0..s.len()] == s || k[1..s.len()+1] == s {
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
