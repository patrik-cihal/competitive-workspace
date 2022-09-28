//{"name":"Problem B - Pot","group":"Kattis - UP_PA","url":"https://open.kattis.com/contests/z9oe6v/problems/pot","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n212\n1253\n","output":"1953566\n"},{"input":"5\n23\n17\n43\n52\n22\n","output":"102\n"},{"input":"3\n213\n102\n45\n","output":"10385\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ProblemBPot"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut result = vec![0; n];

    for i in 0..n {
        let a: i32 = input.read();
        result[i] = (a/10).pow((a%10) as u32);
    }

    out_line!(result.iter().sum::<i32>());
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
