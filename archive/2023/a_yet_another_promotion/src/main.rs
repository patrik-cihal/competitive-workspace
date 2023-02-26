//{"name":"A. Yet Another Promotion","group":"Codeforces - Codeforces Round #852 (Div. 2)","url":"https://codeforces.com/contest/1793/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5 4\n3 1\n5 4\n3 2\n3 4\n3 5\n20 15\n10 2\n1000000000 900000000\n1000000000 8\n","output":"9\n10\n9\n135\n888888888900000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AYetAnotherPromotion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a: usize = input.read();
    let b: usize = input.read();

    let n: usize = input.read();
    let m: usize = input.read();

    let promotions = n/(m+1);
    let candidate1= (promotions*m)*a + b.min(a)*(n-promotions*(m+1));
    let candidate2 = b*n;
    out_line!(candidate1.min(candidate2));
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
