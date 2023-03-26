//{"name":"C. Corners","group":"Codeforces - Codeforces Round 815 (Div. 2)","url":"https://codeforces.com/contest/1720/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 3\n101\n111\n011\n110\n3 4\n1110\n0111\n0111\n2 2\n00\n00\n2 2\n11\n11\n","output":"8\n9\n0\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCorners"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();

    let mut table = vec![0; n].into_iter().map(|_| input.read_digit_string()).collect::<Vec<Vec<u8>>>();

    let mut ones = table.iter().map(|row| row.iter().filter(|x| **x==1).count()).sum::<usize>();

    let mut has_zero = false;
    for i in 0..n-1 {
        for j in 0..m-1 {
            let cur_ones = table[i][j]+table[i+1][j]+table[i][j+1]+table[i+1][j+1];
            if cur_ones <= 2 {
                out_line!(ones);
                return;
            }
            if cur_ones != 4 {
                has_zero = true;
            }
        }
    }
    if !has_zero {
        out_line!(ones-2);
    }
    else {
    out_line!(ones-1);
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
