//{"name":"A. NIT orz!","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n2 3\n3 4\n5 5\n0 2 4 6 8\n1 9\n10\n5 7\n7 15 30 29 27\n3 39548743\n10293834 10284344 13635445\n","output":"7\n13\n11\n31\n48234367\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ANITOrz"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let z: i32 = input.read();
    let v: Vec<i32> = input.read_vec(n);
    let mut ans = 0;
    for a in v {
        ans = ans.max(a|z);
    }
    out_line!(ans);
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
