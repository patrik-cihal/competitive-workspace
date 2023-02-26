//{"name":"binary_combinations","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"binary_combinations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn combinations(n: usize, mut s: Vec<bool>, r: u32, k: u32) -> Vec<Vec<bool>> {
    if n == 0 {
        return vec![s];
    }
    let mut s2 = s.clone();
    s.push(true);
    s2.push(false);

    let mut answer = if r<k {combinations(n-1, s, r+1, k)} else {vec![]};
    answer.append(&mut combinations(n-1, s2, 0, k));
    answer
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k: u32 = input.read();

    let mut answer: Vec<String> = vec![];
    let combinations = combinations(n, vec![], 0, k);

    for comb in combinations {
        answer.push(comb.iter().map(|x| if *x {'1'} else {'0'}).collect())
    }

    out_line!(answer);
    out_line!(answer.len());

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
