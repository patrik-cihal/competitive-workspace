//{"name":"C. Two Brackets","group":"Codeforces - Educational Codeforces Round 98 (Rated for Div. 2)","url":"https://codeforces.com/contest/1452/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n()\n[]()\n([)]\n)]([\n)[(]\n","output":"1\n2\n2\n0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTwoBrackets"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let s = input.read::<String>().chars().collect::<Vec<_>>(); 

    let mut b = 0; 
    let mut sb = 0;

    let mut lb = 0;
    let mut lsb = 0;

    for i in 0..s.len() {
        match s[i] {
            '(' => {
                lb += 1;
            }
            ')' => {
                if lb > 0 {
                    lb -= 1;
                    b += 1;
                }
            }
            '[' => {
                lsb += 1;
            }
            ']' => {
                if lsb > 0{ 
                    lsb-=1;
                    sb +=1;
                }
            }
            _ => panic!()
        }
    }
    out_line!(b+sb);
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
