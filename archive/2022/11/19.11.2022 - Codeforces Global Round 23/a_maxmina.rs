//{"name":"A. Maxmina","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n3 2\n0 1 0\n5 3\n1 0 1 1 0\n2 2\n1 1\n4 4\n0 0 0 0\n6 3\n0 0 1 0 0 1\n7 5\n1 1 1 1 1 1 1\n5 3\n0 0 1 0 0\n","output":"YES\nYES\nYES\nNO\nYES\nYES\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AMaxmina"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: usize = input.read();

    let a = input.read_vec::<i32>(n);
    let mut c = false;
    for i in 0..a.len() {
        if a[i] == 1 {
            c = true;
        }
    }
    out_line!(if c {"YES"} else {"NO"});
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
