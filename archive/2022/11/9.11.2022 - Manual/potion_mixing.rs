//{"name":"potion_mixing","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"potion_mixing"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let s = input.read_vec::<i32>(n);

    let mut g = vec![vec![]; n];

    let mut dp = vec![-1; n];

    for _ in 0..m {
        let p = input.read::<usize>()-1;
        let q= input.read::<usize>()-1;
        g[q].push(p);
    }

    let mut go = RecursiveFunction::new(|f, v: usize| {
        if dp[v] == -1 {
            dp[v] = g[v].iter().map(|&x| f.call( x)).max().unwrap_or(0);
        }
        s[v]+dp[v]
    });


    for i in 0..n {
        go.call(i);
    }

    out_line!(dp);
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
