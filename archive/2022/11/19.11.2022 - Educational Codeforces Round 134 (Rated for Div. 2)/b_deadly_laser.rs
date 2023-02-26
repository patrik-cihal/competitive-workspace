//{"name":"B. Deadly Laser","group":"Codeforces - Educational Codeforces Round 134 (Rated for Div. 2)","url":"https://codeforces.com/contest/1721/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2 3 1 3 0\n2 3 1 3 1\n5 5 3 4 1\n","output":"3\n-1\n8\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BDeadlyLaser"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let sx = input.read::<usize>()-1;
    let sy = input.read::<usize>()-1;

    let d = input.read();

    if (sx <= d && sy <= d) || (n-sx-1 <= d && m-sy-1 <= d) || (m-sy-1).max(sy) <= d || (n-sx-1).max(sx) <= d {
        out_line!(-1);
    }else {
        out_line!(n+m-2);
    }
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
