//{"name":"A. Likes","group":"Codeforces - Codeforces Round 857 (Div. 2)","url":"https://codeforces.com/contest/1802/problem/0","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3\n1 2 -2\n2\n1 -1\n6\n4 3 -1 2 1 -2\n5\n4 2 -2 1 3\n7\n-1 6 -4 3 2 4 1\n","output":"1 2 1\n1 0 1\n1 0\n1 0\n1 2 3 4 3 2\n1 0 1 0 1 2\n1 2 3 4 3\n1 0 1 2 3\n1 2 3 4 5 4 3\n1 0 1 0 1 2 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ALikes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let v = input.read_vec::<i32>(n);

    let mut d = 0;
    let mut l = 0;

    for i in 0..n {
        if v[i] > 0 {
            l += 1;
        }
        else {
            d += 1;
        }
    }

    let mut like_seq = vec![];
    for i in 0..l {
        like_seq.push(1);
    }
    for i in 0..d {
        like_seq.push(-1);
    }

    let mut dislike_seq = vec![];

    for i in 0..d {
        dislike_seq.push(1);
        dislike_seq.push(-1);
    }

    for i in 0..l-d {
        dislike_seq.push(1);
    }

    let mut like_ans = vec![like_seq[0]];
    let mut dislike_ans = vec![dislike_seq[0]];
    for i in 1..n {
        like_ans.push(like_seq[i]+like_ans[i-1]);
        dislike_ans.push(dislike_seq[i]+dislike_ans[i-1]);
    }

    out_line!(like_ans);
    out_line!(dislike_ans);
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
