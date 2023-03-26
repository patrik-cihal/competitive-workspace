//{"name":"B. Mex Master","group":"Codeforces - Codeforces Round 858 (Div. 2)","url":"https://codeforces.com/contest/1806/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n2\n0 0\n3\n0 0 1\n8\n1 0 0 0 2 0 3 0\n","output":"1\n0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMexMaster"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut a = input.read_vec::<u32>(n);

    a.sort();

    let mut zeros = a.iter().filter(|x| **x==0).count();

    let mut ones = a.iter().filter(|x| **x == 1).count();

    if zeros <= (n+1)/2 {
        out_line!(0);
    }
    else {
        if ones != 0 && ones == n-zeros {
            out_line!(2);
        }
        else {
            out_line!(1);
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
