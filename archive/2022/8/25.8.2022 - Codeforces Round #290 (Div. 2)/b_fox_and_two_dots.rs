//{"name":"B. Fox And Two Dots","group":"Codeforces - Codeforces Round #290 (Div. 2)","url":"https://codeforces.com/problemset/problem/510/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\nAAAA\nABCA\nAAAA\n","output":"Yes\n"},{"input":"3 4\nAAAA\nABCA\nAADA\n","output":"No\n"},{"input":"4 4\nYYYR\nBYBY\nBBBY\nBBBY\n","output":"Yes\n"},{"input":"7 6\nAAAAAB\nABBBAB\nABAAAB\nABABBB\nABAAAB\nABBBAB\nAAAAAB\n","output":"Yes\n"},{"input":"2 13\nABCDEFGHIJKLM\nNOPQRSTUVWXYZ\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BFoxAndTwoDots"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut cid = 0;
    // create matrix of chars
    // whenever two same chars are next to each other, spread the id
    // if new one is found create another id
    // once two next to each other with same id are found, we found a cycle
    let mut v = vec![];
    let mut ids = vec![vec![None; m]; n];
    for _ in 0..n {
        v.push(input.read::<String>().chars().collect::<Vec<char>>());
    }

    for i in 0..n {

    }
    out_line!("No");


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
