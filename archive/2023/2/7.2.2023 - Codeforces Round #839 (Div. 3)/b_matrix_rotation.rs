//{"name":"B. Matrix Rotation","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n1 3\n5 7\n8 10\n3 4\n8 10\n4 3\n6 1\n9 2\n7 5\n4 2\n1 2\n4 3\n","output":"YES\nYES\nNO\nYES\nYES\nNO\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMatrixRotation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};


fn rotate(a: &mut i32, b: &mut i32, c: &mut i32, d: &mut i32) {
    let new_a = *c;
    let new_b = *a;
    let new_c = *d;
    let new_d = *b; 
    *a = new_a;
    *b = new_b;
    *c = new_c;
    *d = new_d;
}


fn solve(input: &mut Input, _test_case: usize) {
    let mut a = input.read();    
    let mut b = input.read();
    let mut c = input.read();
    let mut d = input.read();


    for i in 0..4 {
        if a < b && b < d && c<d && a<c {
            out_line!("YES");
            return;
        }
        rotate(&mut a, &mut b, &mut c, &mut d);
    }
    out_line!("NO");
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
