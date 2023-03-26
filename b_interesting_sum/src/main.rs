//{"name":"B. Interesting Sum","group":"Codeforces - Codeforces Round 815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n8\n1 2 2 3 1 5 6 1\n5\n1 2 3 100 200\n4\n3 3 3 3\n6\n7 8 3 1 1 8\n","output":"9\n297\n0\n14\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BInterestingSum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut a = input.read_vec::<u64>(n);
    a.sort();

    let mut answer = a[n-1]+a[n-2]-a[0]-a[1];
    out_line!(answer);

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
