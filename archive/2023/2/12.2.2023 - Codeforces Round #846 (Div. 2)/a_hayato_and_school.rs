//{"name":"A. Hayato and School","group":"Codeforces - Codeforces Round #846 (Div. 2)","url":"https://codeforces.com/contest/1780/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n3\n1 1 1\n4\n1 1 2 2\n3\n1 2 3\n5\n1 4 5 1 2\n4\n2 6 2 4\n5\n5 6 3 2 1\n","output":"YES\n1 2 3\nYES\n3 4 1\nNO\nYES\n1 3 4\nNO\nYES\n1 3 5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AHayatoAndSchool"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let a = input.read_vec::<u32>(n);
    let mut odd = vec![];
    let mut even = vec![];
    for i in 0..n {
        if a[i]%2 == 1 {
            odd.push(i);
        }
        else {
            even.push(i);
        }
    }
    if odd.len() >= 3 {
        out_line!("YES");
        out_line!(odd[0]+1, odd[1]+1, odd[2]+1);
    }
    else if odd.len() > 0 && even.len() > 1 {
        out_line!("YES");
        out_line!(odd[0]+1, even[0]+1, even[1]+1);
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
