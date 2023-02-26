//{"name":"C. Dora and Search","group":"Codeforces - Codeforces Round #852 (Div. 2)","url":"https://codeforces.com/contest/1793/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n3\n1 2 3\n4\n2 1 4 3\n7\n1 3 2 4 6 5 7\n6\n2 3 6 5 4 1\n","output":"-1\n1 4\n2 6\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDoraAndSearch"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let a = input.read_vec::<usize>(n);

    let mut mn = 1;
    let mut mx = n;

    let mut start = 0;
    let mut end = n-1; 

    while start<end {
        if a[start] == mn {
            start += 1;
            mn += 1;
        }
        else if a[start] == mx {
            start += 1;
            mx -=1;
        }
        else if a[end] == mn {
            mn += 1;
            end -=1;
        }
        else if a[end] == mx {
            mx -=1;
            end -=1;
        }
        else {
            break;
        }
    }

    if start<end {
        out_line!(start+1, end+1);
    }
    else {
        out_line!(-1);
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
