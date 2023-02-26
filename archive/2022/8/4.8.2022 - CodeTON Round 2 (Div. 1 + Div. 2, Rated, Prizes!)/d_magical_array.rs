//{"name":"D. Magical Array","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n3 9\n0 1 2 0 0 2 1 1 0\n0 1 1 1 2 0 0 2 0\n0 1 2 0 0 1 2 1 0\n3 7\n25 15 20 15 25 20 20\n26 14 20 14 26 20 20\n25 15 20 15 20 20 25\n3 9\n25 15 20 15 25 20 20 20 20\n26 14 20 14 26 20 20 20 20\n25 15 20 15 25 15 20 20 25\n3 11\n25 15 20 15 25 20 20 20 20 20 20\n26 14 20 14 26 20 20 20 20 20 20\n25 15 20 15 25 20 15 20 20 20 25\n3 13\n25 15 20 15 25 20 20 20 20 20 20 20 20\n26 14 20 14 26 20 20 20 20 20 20 20 20\n25 15 20 15 25 20 20 15 20 20 20 20 25\n3 15\n25 15 20 15 25 20 20 20 20 20 20 20 20 20 20\n26 14 20 14 26 20 20 20 20 20 20 20 20 20 20\n25 15 20 15 25 20 20 20 15 20 20 20 20 20 25\n3 9\n909459 479492 676924 224197 162866 164495 193268 742456 728277\n948845 455424 731850 327890 304150 237351 251763 225845 798316\n975446 401170 792914 272263 300770 242037 236619 334316 725899\n","output":"3 1\n3 10\n3 15\n3 20\n3 25\n3 30\n1 1378716\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMagicalArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();

    let mut h = vec![0; n];
    for i in 0..n {
        let v: Vec<i64> = input.read_vec(m);
        h[i] = v.iter().enumerate().map(|(i, x)| { x*i as i64 }).sum();
    }

    let mut rh = h[0];
    if h[0] != h[1] {
        rh = h[2];
    }
    for i in 0..n {
        if h[i] != rh {
            out_line!(i+1, h[i]-rh);
            return;
        }
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
