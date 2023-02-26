//{"name":"B. Death's Blessing","group":"Codeforces - Educational Codeforces Round 138 (Rated for Div. 2)","url":"https://codeforces.com/contest/1749/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n10\n0\n3\n100 1 100\n1 100 1\n4\n2 6 7 3\n3 6 0 5\n2\n1000000000 1000000000\n1000000000 1000000000\n","output":"10\n203\n26\n3000000000\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDeathsBlessing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a = input.read_vec::<u64>(n);
    let b = input.read_vec::<u64>(n);

    let sum_a: u64 = a.iter().sum();
    let sum_b: u64 = b.iter().sum();

    out_line!(sum_a+sum_b-*b.iter().max().unwrap());

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
