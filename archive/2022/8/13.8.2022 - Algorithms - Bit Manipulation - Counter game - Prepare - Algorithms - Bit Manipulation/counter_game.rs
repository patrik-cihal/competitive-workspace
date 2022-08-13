//{"name":"Counter game","group":"HackerRank - Algorithms - Bit Manipulation - Counter game - Prepare - Algorithms - Bit Manipulation","url":"https://www.hackerrank.com/challenges/counter-game/problem?isFullScreen=true&h_r=next-challenge&h_v=zen&h_r=next-challenge&h_v=zen","interactive":false,"timeLimit":4000,"tests":[{"input":"1\n6\n","output":"Richard\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CounterGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut n: u64 = input.read();
    let mut ans = 0;
    while n%2 != 1 {
        n >>= 1;
        ans += 1;
    }
    while n!=0 {
        n >>= 1;
        ans += n%2;
    }
    out_line!(if ans%2==1 {"Louise"} else {"Richard"});
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
