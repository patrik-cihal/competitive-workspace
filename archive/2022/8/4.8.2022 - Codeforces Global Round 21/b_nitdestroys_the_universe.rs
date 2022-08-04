//{"name":"B. NIT Destroys the Universe","group":"Codeforces - Codeforces Global Round 21","url":"https://codeforces.com/contest/1696/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4\n0 0 0 0\n5\n0 1 2 3 4\n7\n0 2 3 0 1 2 0\n1\n1000000000\n","output":"0\n1\n2\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNITDestroysTheUniverse"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v: Vec<i32> = input.read_vec(n);
    let mut ans = 0;
    let mut l = 0;
    for i in 0..n {
        if v[i] != 0 {
            break;
        }
        l+=1;
    }
    let mut r = n;
    for i in (0..n).rev() {
        if v[i] != 0 {
            break;
        }
        r-=1;
    }
    if r==0 {
        out_line!(0);
        return;
    }

    for i in l..r {
        if v[i] == 0 && v[i-1] != 0 {
            ans += 1;
        }
    }
    out_line!((ans+1).min(2));
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
