//{"name":"A. Image","group":"Codeforces - Educational Codeforces Round 134 (Rated for Div. 2)","url":"https://codeforces.com/contest/1721/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nrb\nbr\ncc\nwb\naa\naa\nab\ncd\nyy\nxx\n","output":"1\n2\n0\n3\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AImage"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let us = input.read::<String>().chars().collect::<Vec<char>>();
    let ds = input.read::<String>().chars().collect::<Vec<char>>();

    let a = 'a' as usize;
    let mut counts = vec![0; 26];
    counts[us[0] as usize - a] = 1;
    counts[us[1] as usize - a] = 1;
    counts[ds[1] as usize -a] = 1;
    counts[ds[0] as usize -a] =1;
    out_line!(counts.iter().sum::<i32>()-1);
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
