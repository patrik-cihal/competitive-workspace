//{"name":"C. Maximum Set","group":"Codeforces - Educational Codeforces Round 144 (Rated for Div. 2)","url":"https://codeforces.com/contest/1796/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 11\n13 37\n1 22\n4 100\n","output":"2 4\n2 6\n5 1\n5 7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMaximumSet"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let (l, r): (usize, usize) = input.read();

    let mut cur = l;
    let mut max_count = 0; 

    while cur<=r {
        max_count += 1;
        cur *= 2;
    }

    cur /= 2;

    let mut answer = 1;
    let offset = 2usize.pow((max_count-1)as u32);

    answer += (r-cur)/offset;


    if cur / 2 * 3 <= r {
        answer += max_count-1;
        let offset = 2usize.pow((max_count-2) as u32) * 3;
        let cur = cur/2*3;
        for _ in 0..max_count-1 {
            answer = (answer + (r-cur)/offset)%998244353;
        }
    }
    
    out_line!(max_count, answer%998244353);
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
