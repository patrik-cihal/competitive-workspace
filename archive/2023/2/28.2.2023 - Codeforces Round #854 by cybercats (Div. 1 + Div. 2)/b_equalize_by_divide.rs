//{"name":"B. Equalize by Divide","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1\n100\n3\n1 1 1\n2\n2 1\n2\n5 5\n3\n4 3 2\n4\n3 3 4 4\n2\n2 100\n5\n5 3 6 7 8\n6\n3 3 80 3 8 3\n4\n19 40 19 55\n","output":"0\n0\n-1\n0\n2\n1 3\n2 1\n4\n3 1\n4 2\n1 3\n2 4\n6\n2 1\n2 1\n2 1\n2 1\n2 1\n2 1\n8\n5 2\n4 2\n3 2\n1 3\n1 3\n2 1\n4 1\n5 1\n4\n5 1\n3 1\n3 1\n3 1\n9\n4 2\n2 1\n1 2\n1 2\n3 2\n3 2\n1 4\n2 4\n3 4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEqualizeByDivide"}}}

use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn is_equal(a: &Vec<(u32, usize)>) -> bool {

    let mut equal = true;
    for i in 1..a.len() {
        if a[i].0 != a[i-1].0 {
            equal = false;
        }
    }
    equal
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let mut a = input.read_vec::<u32>(n).into_iter().enumerate().map(|(i, x)| (x, i)).collect::<Vec<_>>();

    a.sort();
    if a[0].0 == 1 && !is_equal(&a) {
        out_line!(-1);
        return;
    }

    if n == 1 {
        out_line!(0);
        return;
    }


    let mut answer = vec![];
    while !is_equal(&a) {
        for i in 1..n {
            while a[i].0 > a[i-1].0 {
                a[i].0 = (a[i].0+a[i-1].0-1)/a[i-1].0;
                answer.push((a[i].1, a[i-1].1));
            }
        }
        a.sort();
    }


    out_line!(answer.len());
    for operation in answer {
        out_line!(operation.0 + 1, operation.1 + 1);
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
