//{"name":"A. Recent Actions","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 1\n2\n3 2\n5 4\n4 5\n5 9 9 5 7\n5 5\n6 7 8 9 10\n3 4\n4 4 4 4\n4 4\n5 5 6 6\n3 5\n4 5 5 5 4\n4 20\n5 5 24 24 24 5 6 7 8 9 10 12 13 14 15 16 17 18 19 20\n5 7\n7 8 7 11 7 12 10\n6 7\n8 11 7 8 8 8 12\n","output":"1\n-1 2 1\n-1 5 2 1\n5 4 3 2 1\n-1 -1 1\n-1 -1 3 1\n-1 2 1\n8 7 3 1\n7 6 4 2 1\n-1 -1 7 3 2 1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ARecentActions"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let m: usize = input.read();

    let mut a: Vec<usize> = input.read_vec(m);


    let mut seen = vec![false; n+m+10];

    let mut answer = vec![];
    for i in 0..m {
        if !seen[a[i]] {
            answer.push(i+1);
            seen[a[i]] = true;
        }
    }

    answer = answer[0..n.min(answer.len())].to_vec();
    answer.reverse();

    for i in answer.len()..n {
        out!(-1, "");
    }
    for i in 0..answer.len().min(n) {
        out!(answer[i], "");
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
