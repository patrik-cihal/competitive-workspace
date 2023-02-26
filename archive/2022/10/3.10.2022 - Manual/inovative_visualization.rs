//{"name":"inovative_visualization","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"inovative_visualization"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const MOD: u64 = 1_000_000_000 + 7;

fn fact(a: u64) -> u64 {
    if a==0 {
        1
    }
    else {
        a * fact(a-1) % MOD
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    if n == 1 {
        out_line!(1);
        return;
    }

    let mut cc = vec![0; n];

    for _ in 1..n {
        let j = input.read::<usize>()-1;
        cc[j] += 1;
    }

    let mut answer = n as u64 * fact(cc[0]);
    for i in 1..n {
        answer *= (cc[i]+1) * fact(cc[i]) % MOD;
    }

    out_line!(answer%MOD);
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
