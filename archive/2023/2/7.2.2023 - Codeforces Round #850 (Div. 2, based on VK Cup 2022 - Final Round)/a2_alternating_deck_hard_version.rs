//{"name":"A2. Alternating Deck (hard version)","group":"Codeforces - Codeforces Round #850 (Div. 2, based on VK Cup 2022 - Final Round)","url":"https://codeforces.com/contest/1786/problem/A2","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10\n6\n17\n8\n1000000\n","output":"3 2 2 3\n1 0 2 3\n6 4 3 4\n2 1 2 3\n250278 249924 249722 250076\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"A2AlternatingDeckHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a = [0, 0]; 
    let mut b = [0, 0];


    for i in 1.. {
        let sum = a[0]+b[0]+a[1]+b[1];
        if sum >= n {
            break;
        }
        let d = (n-sum).min(i);

        if i%4 == 1 {
            a[0] += (d+1)/2;
            a[1] += d/2;
        }
        else if i%4 == 2 {
            b[1] += (d+1)/2;
            b[0] += d/2;
        }
        else if i%4 == 3 {
            b[1] += (d+1)/2;
            b[0] += d/2;
        }
        else {
            a[0] += (d+1)/2;
            a[1] += d/2;
        }
    }
    
    out_line!(a[0], a[1], b[0], b[1]);


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
