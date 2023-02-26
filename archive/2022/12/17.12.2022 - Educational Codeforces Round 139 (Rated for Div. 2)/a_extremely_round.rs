//{"name":"A. Extremely Round","group":"Codeforces - Educational Codeforces Round 139 (Rated for Div. 2)","url":"https://codeforces.com/contest/1766/problem/A","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n9\n42\n13\n100\n111\n","output":"9\n13\n10\n19\n19\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AExtremelyRound"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut n: u32 = input.read();
    let k = (n as f64).log10() as u32;
    let mut answer =  (k*10);
    let c = 10u32.pow(k);
    while n>=c {
        n-=c;
        answer += 1;
    }

    out_line!(answer-k);

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
