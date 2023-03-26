//{"name":"F. Bouncy Ball","group":"Codeforces - Codeforces Round 859 (Div. 4)","url":"https://codeforces.com/contest/1807/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n5 7 1 7 2 4 DL\n5 7 1 7 3 2 DL\n3 3 1 3 2 2 UR\n2 4 2 1 2 2 DR\n4 3 1 1 1 3 UL\n6 4 1 2 3 4 DR\n","output":"3\n-1\n1\n-1\n4\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FBouncyBall"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();

    let mut dp = vec![vec![vec![vec![false; 2]; 2]; m]; n];

    let (sr, sc, tr, tc): (usize, usize, usize, usize) = input.read();
    let (sr, sc, tr, tc) = (sr-1, sc-1, tr-1, tc-1);

    let dir = input.read::<String>();
    let dir_str = dir.as_str();

    let mut dir = match dir_str {
        "DL" => [1, 0],
        "DR" => [1, 1],
        "UR" => [0, 1],
        "UL" => [0, 0],
        _ => panic!()
    };

    let (mut cr, mut cc) = (sr, sc);
    let mut answer = 0;
    while !dp[cr][cc][dir[0]][dir[1]] {
        if cr == tr && cc == tc {
            out_line!(answer);
            return;
        }
        dp[cr][cc][dir[0]][dir[1]] = true;
        let mut bounce = false;
        if cr == n-1 && dir[0] == 1 {
            dir[0] = 0; 
            bounce = true;
        }
        if cc == m-1 && dir[1] == 1 {
            dir[1] = 0;
            bounce = true;
        }
        if cr == 0 && dir[0] == 0 {
            dir[0] = 1; 
            bounce = true;
        }
        if cc == 0 && dir[1] == 0 {
            dir[1] = 1;
            bounce = true;
        }
        if bounce {
            answer += 1;
        }
        cr = (cr as i32 + dir[0]as i32*2-1) as usize;
        cc = (cc as i32 +  dir[1]as i32*2-1) as usize;
    }
    out_line!(-1);
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
