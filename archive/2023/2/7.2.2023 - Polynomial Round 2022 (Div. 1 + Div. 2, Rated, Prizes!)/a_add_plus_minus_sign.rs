//{"name":"A. Add Plus Minus Sign","group":"Codeforces - Polynomial Round 2022 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1774/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n11\n5\n01101\n5\n10001\n","output":"-\n+-++\n+++-\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAddPlusMinusSign"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();

    let s: Vec<char> = input.read::<String>().chars().collect();
    let mut c = false;
    for val in s[1..].iter() {
        if val == &'1' {
            if c {
                out!('+');
            }
            else {
                out!('-');
            }
            c = !c;
        }
        else {
            out!('+');
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
