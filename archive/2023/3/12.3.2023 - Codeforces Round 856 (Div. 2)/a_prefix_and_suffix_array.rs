//{"name":"A. Prefix and Suffix Array","group":"Codeforces - Codeforces Round 856 (Div. 2)","url":"https://codeforces.com/contest/1794/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n4\nbcd cd a d abc ab\n3\ni io i oi\n2\ng g\n3\nt al lt a\n4\nbba a ab a abb ba\n","output":"NO\nYES\nYES\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APrefixAndSuffixArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut ps: Vec<Vec<char>> = input.read_vec::<String>(2*n-2).into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect();
    ps.sort_by(|a, b| a.len().cmp(&b.len()));

    let mut last = ps.pop().unwrap();
    last.reverse();
    if last == ps.pop().unwrap() {
        out_line!("YES");
        return;
    }
    else {
        out_line!("NO");
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
