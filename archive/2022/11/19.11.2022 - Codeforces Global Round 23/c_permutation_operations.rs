//{"name":"C. Permutation Operations","group":"Codeforces - Codeforces Global Round 23","url":"https://codeforces.com/contest/1746/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n1 2 3 4\n5\n1 3 2 4 5\n3\n2 3 1\n1\n1\n","output":"1 1 1 1\n1 4 3 2 1\n1 3 3\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPermutationOperations"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);

    let mut splits = vec![];
    for i in 1..n {
        if v[i-1]>v[i] {
            splits.push((v[i-1]-v[i], i+1));
        }
    }

    splits.sort();

    for i in 0..n-splits.len() {
        out!(1, "");
    }
    for i in 0..splits.len() {
        out!(splits[i].1, "");
    }
    out_line!();
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
