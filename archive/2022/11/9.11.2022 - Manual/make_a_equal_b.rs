//{"name":"make_a_equal_b","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"make_a_equal_b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut a  = input.read_vec::<u32>(n);
    let mut b  = input.read_vec::<u32>(n);

    let mut d1 = 0;
    for i in 0..n {
        if a[i] != b[i] {
            d1 += 1;
        }
    }

    let mut d2 = 0;
    a.sort();
    b.sort();
    for i in 0..n {
        if a[i] != b[i] {
            d2 += 1;
        }
    }

    out_line!(d1.min(d2+1));
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
