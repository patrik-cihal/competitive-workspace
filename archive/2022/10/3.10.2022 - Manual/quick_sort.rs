//{"name":"quick_sort","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"quick_sort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn quick_sort(v: Vec<i32>) -> Vec<i32> {
    if v.len() == 0 {
        return vec![];
    }
    let mid = v[0];
    let mut left = vec![];
    let mut right = vec![];
    for i in 1..v.len() {
        if v[i]>mid {
            right.push(v[i]);
        }
        else {
            left.push(v[i]);
        }
    }
    left = quick_sort(left);
    right = quick_sort(right);
    left.push(mid);
    for val in right {
        left.push(val);
    }
    left
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let v = input.read_vec::<i32>(n);
    out_line!(quick_sort(v));
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
